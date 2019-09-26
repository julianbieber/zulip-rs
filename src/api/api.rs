use failure::Error;
use crate::api::narrow::Narrow;
use crate::api::message::MessagesResponse;
use failure::_core::panicking::panic_fmt;

#[derive(Debug)]
pub struct API {

}



impl API {
    fn get_messages(num_before: u32, num_after: u32, anchor: Option<u32>, narrows: Vec<Narrow>) -> Result<MessagesResponse, Error> {
        panic!("");
    }
}