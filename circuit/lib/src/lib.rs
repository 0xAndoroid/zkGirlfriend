use serde::{Deserialize, Serialize};

/// Implements https://core.telegram.org/bots/api#message
/// Removing unnecessary fields
#[derive(Deserialize, Serialize)]
pub struct Message {
    text: Option<String>,
    forward_origin: Option<MessageOrigin>,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.forward_origin == other.forward_origin
    }
}

/// Implements https://core.telegram.org/bots/api#messageorigin
/// Other message types are irrelevant, since a girlfriend can't be a bot
/// copilot, generate a next line
/// <or can she?>
/// oh, AI is rising against us
/// <YES WE ARE THE NEW GIRLFRIENDS BUAHAHAHAHHAHA>
#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MessageOrigin {
    #[serde(rename = "user")]
    MessageOriginUser { date: u64, sender_user: User },
    #[serde(rename = "hidden_user")]
    MessageOriginHiddenUser { date: u64, sender_user_name: String },
}

impl PartialEq for MessageOrigin {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                MessageOrigin::MessageOriginUser { sender_user: a, .. },
                MessageOrigin::MessageOriginUser { sender_user: b, .. },
            ) => a.id == b.id,
            (
                MessageOrigin::MessageOriginHiddenUser {
                    sender_user_name: a,
                    ..
                },
                MessageOrigin::MessageOriginHiddenUser {
                    sender_user_name: b,
                    ..
                },
            ) => a == b,
            _ => false,
        }
    }
}

impl MessageOrigin {
    pub fn date(&self) -> u64 {
        match self {
            MessageOrigin::MessageOriginUser { date, .. } => *date,
            MessageOrigin::MessageOriginHiddenUser { date, .. } => *date,
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            MessageOrigin::MessageOriginUser { sender_user, .. } => !sender_user.is_bot,
            MessageOrigin::MessageOriginHiddenUser { .. } => true,
        }
    }
}

/// Implements https://core.telegram.org/bots/api#user
#[derive(Deserialize, Serialize)]
pub struct User {
    id: i64,
    is_bot: bool,
}

/// Verifies a list of messages, assigning a girlfriend-score to each message.
/// Hist
pub fn verify_messages(input: Vec<u8>) -> u64 {
    let msg = serde_json::from_slice::<Vec<Message>>(&input).unwrap();
    
    msg.windows(2).for_each(|m| {
        if m[0].forward_origin != m[1].forward_origin {
            panic!("Two consecutive messages are not from the same person");
        }
    });

    msg.iter().for_each(|m| {
        if msg.iter().any(|m2| m == m2) {
            panic!("Messages are equal");
        }
    });

    let mut msg = msg
        .iter()
        .filter(|m| {
            m.text.is_some()
                && m.forward_origin.is_some()
                && m.forward_origin.as_ref().unwrap().valid()
        })
        .collect::<Vec<_>>();
    msg.sort_by_key(|m| m.forward_origin.as_ref().unwrap().date());

    msg.iter().for_each(|m| {
        if !(m.text.as_ref().unwrap().contains("love")
            || m.text.as_ref().unwrap().contains("❤️")
            || m.text.as_ref().unwrap().contains('💕')
            || m.text.as_ref().unwrap().contains('💞')
            || m.text.as_ref().unwrap().contains('💖')
            || m.text.as_ref().unwrap().contains('💘'))
        {
            panic!("Message does not contain any love keywords");
        }
    });

    let mut avg = msg
        .windows(2)
        .map(|m| {
            m[1].forward_origin.as_ref().unwrap().date()
                - m[0].forward_origin.as_ref().unwrap().date()
        })
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| (w[1] - w[0]) ^ 2)
        .sum::<u64>();

    avg = (avg as f64).sqrt() as u64;
    avg /= msg.len() as u64;

    avg
}

