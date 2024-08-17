use serde::{Deserialize, Serialize};

/// Implements https://core.telegram.org/bots/api#message
/// Removing unnecessary fields
#[derive(Deserialize, Serialize, Debug)]
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
#[derive(Deserialize, Serialize, Debug)]
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
#[derive(Deserialize, Serialize, Debug)]
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

    msg.iter().enumerate().for_each(|(i, m)| {
        if msg.iter().enumerate().any(|(j, m2)| m == m2 && i != j) {
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

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_verify_messages() {
        let input = r#"
        [{"text":"❤️❤️❤️❤️❤️😘😘😘😘😘😘","forward_origin":{"type":"user","date":1723833441,"sender_user":{"id":5936622848,"is_bot":false}}},{"text":"Miss you already. Can't wait to see you tonight 💕","forward_origin":{"type":"user","date":1723833503,"sender_user":{"id":5936622848,"is_bot":false}}},{"text":"You make my world brighter every day! ☀️❤️\n\nCan’t wait to see you later! 😘💖\nThinking of you always, my love! 💭💕\nYou’re my favorite part of every day! 😍🌸\nJust wanted to say I love you! 🥰❤️\nMissing you more than usual today 😢❤️\nYou make everything better 🥰🌟\nMy heart belongs to you, always ❤️🔒\nEvery moment with you is magic ✨💖\nYou’re my everything! 🌍💞","forward_origin":{"type":"user","date":1723846580,"sender_user":{"id":5936622848,"is_bot":false}}}]
        "#;

        assert_eq!(verify_messages(input.as_bytes().to_vec()), 1);
    }
}
