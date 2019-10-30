use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub anchor: Option<i64>,
    pub found_newest: Option<bool>,
    pub found_oldest: Option<bool>,
    pub found_anchor: Option<bool>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub content_type: String,
    pub id: i64,
    pub sender_email: String,
    pub sender_full_name: String,
    pub sender_id: i64,
    pub stream_id: i64,
    pub subject: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEvent {
    pub id: i64,
    pub message: Message
}

#[derive(Debug)]
pub struct PostResponse {
    pub id: i64
}
