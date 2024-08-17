use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::Message;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(BotCommands, Clone)]
#[command(rename_all = "snake_case")]
enum Command {
    Help,
    Start,
    Verify,
}

#[derive(Serialize, Deserialize)]
struct ProofMessage {
    forward_origin: ForwardOrigin,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct ForwardOrigin {
    sender_user: SenderUser,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct SenderUser {
    username: String,
}

struct Data {
    chat_message_proofs: HashMap<i64, Vec<ProofMessage>>,
    chat_message_ids: HashMap<i64, Vec<u64>>,
    chat_message_girlfriend: HashMap<i64, String>,
}

impl Data {
    fn new() -> Self {
        Data {
            chat_message_proofs: HashMap::new(),
            chat_message_ids: HashMap::new(),
            chat_message_girlfriend: HashMap::new(),
        }
    }
}

async fn help_handler(bot: Bot, msg: Message, data: Data) {
    let chat_id = msg.chat.id;
    let response = "Prove you have a Girlfriend by forwarding a minimum of 3 messages from her.\nUse /start to begin the process.\nUse /verify to start the Zero-Knowledge Proof process.\nThe more messages you forward, with evenly spread out dates the better!\nAccepted Regex Matches:\n\t- \"i love you\"\n\t- \"❤️\"";
    bot.send_message(chat_id, response).await.unwrap();
}

async fn start_handler(bot: Bot, msg: Message, mut data: Data) {
    let chat_id = msg.chat.id;
    let response = "Prove you have a Girlfriend by forwarding a minimum of 3 messages from her.\nThe more messages you forward, with evenly spread out dates the better!\nAccepted Regex Matches:\n\t- \"i love you\"\n\t- \"❤️\"";
    data.chat_message_proofs.insert(chat_id, Vec::new());
    data.chat_message_ids.insert(chat_id, Vec::new());
    bot.send_message(chat_id, response).await.unwrap();
}

async fn verify_handler(bot: Bot, msg: Message, data: Data) {
    let chat_id = msg.chat.id;
    let proofs = data.chat_message_proofs.get(&chat_id).unwrap_or(&Vec::new());

    if proofs.len() < 3 {
        bot.send_message(chat_id, "Please forward at least 3 messages to verify.").await.unwrap();
        return;
    }

    // Simulate generating proofs...
    bot.send_message(chat_id, "Generating Zero-Knowledge Proofs...").await.unwrap();
}

async fn respond_handler(bot: Bot, msg: Message, mut data: Data) {
    let chat_id = msg.chat.id;
    let sender_username = msg.forward_from().unwrap().username.clone();
    let message_id = (msg.date as u64) + (msg.text().unwrap().len() as u64); // Simple hash for demo

    if let Some(girlfriend) = data.chat_message_girlfriend.get(&chat_id) {
        if &sender_username != girlfriend {
            bot.send_message(chat_id, format!("Please forward messages from your girlfriend: {}", girlfriend)).await.unwrap();
            return;
        }
    } else {
        data.chat_message_girlfriend.insert(chat_id, sender_username.clone());
    }

    if let Some(ids) = data.chat_message_ids.get(&chat_id) {
        if ids.contains(&message_id) {
            bot.send_message(chat_id, "You have already forwarded this message.").await.unwrap();
            return;
        }
    }

    let proof_message = ProofMessage {
        forward_origin: ForwardOrigin {
            sender_user: SenderUser { username: sender_username.clone() },
            date: msg.date.to_string(),
        },
        text: msg.text().unwrap().to_string(),
    };

    data.chat_message_proofs.entry(chat_id).or_default().push(proof_message);
    data.chat_message_ids.entry(chat_id).or_default().push(message_id);

    bot.send_message(chat_id, format!("Thank you for the forwarded message. You have forwarded {} messages so far.", data.chat_message_proofs.get(&chat_id).unwrap().len())).await.unwrap();
}

async fn regular_handler(bot: Bot, msg: Message, data: Data) {
    let chat_id = msg.chat.id;
    bot.send_message(chat_id, format!("Please forward messages only, or use /start to (re)start the process. You are currently at {} forwarded messages.", data.chat_message_proofs.get(&chat_id).unwrap_or(&Vec::new()).len())).await.unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");

    let bot = Bot::new(token);
    let data = Data::new();

    teloxide::repl(bot, move |bot, msg| {
        let data = data.clone();
        async move {
            match msg.update.text() {
                Some(text) if text.starts_with('/') => {
                    let cmd = text.split_whitespace().next().unwrap_or("");
                    match Command::from_str(cmd) {
                        Command::Help => help_handler(bot.clone(), msg.update.clone(), data.clone()).await,
                        Command::Start => start_handler(bot.clone(), msg.update.clone(), data.clone()).await,
                        Command::Verify => verify_handler(bot.clone(), msg.update.clone(), data.clone()).await,
                    }
                }
                _ if msg.update.forward_from().is_some() => respond_handler(bot.clone(), msg.update.clone(), data.clone()).await,
                _ => regular_handler(bot.clone(), msg.update.clone(), data.clone()).await,
            }
        }
    })
    .await;
}
