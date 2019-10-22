use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub anchor: Option<u32>,
    pub found_newest: bool,
    pub found_oldest: bool,
    pub found_anchor: bool,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub content_type: String,
    pub id: u32,
    pub sender_email: String,
    pub sender_full_name: String,
    pub sender_id: u32,
    pub stream_id: u32,
    pub subject: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEvent {
    pub id: i32,
    pub message: Message
}

#[derive(Debug)]
pub struct PostResponse {
    pub id: u32
}
