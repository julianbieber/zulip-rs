use crate::api::narrow::Narrow;
use failure::Error;

pub struct Queue {
    id: String,
    last_event_id: i32
}
