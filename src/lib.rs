pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::api::API;
    use crate::api::config::*;
    use crate::api::narrow::Narrow;

    #[test]
    fn test_post_and_get_messages() {
        let api = API::from_config(ZULIP_CONFIG.as_ref().expect("Failed to read config"));

        let posted = api.post_message("julian bot test", "zulip-rs", "test_post_and_get_messages").expect("Failed to post messages");

        let messages = api.get_messages(1, 1, Some(posted.id), &[
            Narrow::stream("julian bot test".to_string(), false)
        ]).expect("Failed to request messages");

        assert!(messages.messages.len() >= 1, "Must receive at least the posted message");
    }
}
