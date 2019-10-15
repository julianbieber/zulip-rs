use failure::Fail;


#[derive(Debug, Fail)]
pub enum ZulipApiError {
    #[fail(display = "Failed to post message: {}", message)]
    FailedToPostMessage {
        message: String
    }
}