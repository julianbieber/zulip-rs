use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagesResponse {
    anchor: Option<u32>,
    found_newest: bool,
    found_oldest: bool,
    found_anchor: bool,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    content: String,
    content_type: String,
    id: u32,
    sender_email: String,
    sender_full_name: String,
    sender_id: u32,
    stream_id: u32,
    subject: String,
    timestamp: u64,
}
