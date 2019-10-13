use crate::api::message::MessagesResponse;
use crate::api::narrow::Narrow;
use failure::Error;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::{Client};
use crate::api::config::ZulipConfig;

#[derive(Debug)]
pub struct API {
    zulip_domain: String,
    user: String,
    pass: String,
    client: Client,
}

const _FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

impl API {
    pub fn get_messages(
        &self,
        num_before: u32,
        num_after: u32,
        anchor: Option<u32>,
        narrows: &[Narrow],
    ) -> Result<MessagesResponse, Error> {
        let anchor_parameter = anchor
            .map(|a| ("anchor".to_string(), format!("{}", a)))
            .unwrap_or(("use_first_unread_anchor".to_string(), "true".to_string()));

        let url = if self.zulip_domain.ends_with("/") {
            format!("{}api/v1/messages", &self.zulip_domain)
        } else {
            format!("{}/api/v1/messages", &self.zulip_domain)
        };

        let response = self
            .client
            .get(url.as_str())
            .query(&[
                ("num_before", format!("{}", num_before).as_str()),
                ("num_after", format!("{}", num_after).as_str()),
                (anchor_parameter.0.as_str(), anchor_parameter.1.as_str()),
                ("narrow", serde_json::to_string(narrows)?.as_str())
            ])
            .basic_auth(self.user.as_str(), Some(self.pass.as_str()))
            .send()?
            .json()?;
        Ok(response)
    }

    pub fn new(zulip_domain: String, user: String, pass: String) -> API {
        API {
            zulip_domain,
            user,
            pass,
            client: Client::new(),
        }
    }

    pub fn from_config(config: &ZulipConfig) -> API {
        API::new(config.domain.clone(), config.user.clone(), config.password.clone())
    }
}
