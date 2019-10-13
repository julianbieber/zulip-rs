pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::api::API;
    use crate::api::config::*;
    use crate::api::narrow::Narrow;

    #[test]
    fn test_get_messages() {
        let api = API::from_config(ZULIP_CONFIG.as_ref().expect("Failed to read config"));
        let messages = api.get_messages(1, 1, Some(31882), &[
            Narrow::stream("julian bot test".to_string(), false)
        ]).expect("Failed to request messages");
        println!("{:?}", messages);
    }
}
