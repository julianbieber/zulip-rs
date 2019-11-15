pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::api::API;
    use crate::api::config::*;
    use crate::api::narrow::Narrow;

    const STREAM: &'static str = "julian bot test";
    const TOPIC: &'static str = "zulip-rs";

    #[test]
    fn test_post_and_get_messages() {
        let api = API::from_config(&ZulipConfig::from_file(ZULIP_CONFIG_PATH.to_path_buf()).expect("Failed to read config"));

        let posted = api.post_message(STREAM, TOPIC, "test_post_and_get_messages").expect("Failed to post messages");

        let messages = api.get_messages(1, 1, Some(posted.id), &[
            Narrow::stream("julian bot test".to_string(), false)
        ]).expect("Failed to request messages");

        assert!(messages.messages.len() >= 1, "Must receive at least the posted message");
    }

    #[test]
    fn subscribe_to_messages() {
        let message = "this message should appear in a queue";
        let api = API::from_config(&ZulipConfig::from_file(ZULIP_CONFIG_PATH.to_path_buf()).expect("Failed to read config"));
        let mut queue = api.create_queue(true, &[]).expect("Failed to create queue");
        api.post_message(STREAM, TOPIC, message).expect("Failed to post message");
        let messages = api.get_queued_messages(&mut queue).expect("Failed to retrieve queued messages");
        assert!(messages.len() >= 1, "Must receive at least the posted message");
        assert_eq!(messages.last().unwrap().content, message);
    }

    #[test]
    fn work_with_streams() {
        let api = API::from_config(&ZulipConfig::from_file(ZULIP_CONFIG_PATH.to_path_buf()).expect("Failed to read config"));

        api.create_stream("test zulip-rs stream", false).expect("Failed to create stream");
    }
}
