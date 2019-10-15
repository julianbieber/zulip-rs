use crate::api::message::{MessagesResponse, PostResponse};
use crate::api::narrow::Narrow;
use crate::api::config::ZulipConfig;

use serde::{Deserialize, Serialize};
use failure::Error;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::{Client};
use crate::api::errors::ZulipApiError;

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

        let url = self.build_url("api/v1/messages");

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

    pub fn post_message(&self, stream: &str, topic: &str, message: &str) -> Result<PostResponse, Error> {
        let url = self.build_url("api/v1/messages");
        let response = self.client
            .post(url.as_str())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(self.user.as_str(), Some(self.pass.as_str()))
            .body([
                format!("type=stream"),
                format!("to={}", stream),
                format!("subject={}", topic),
                format!("content={}", message),
            ].join("&"))
            .send()?
            .json::<InternalPostResponse>()?;

        if response.result == "success" {
            Ok(PostResponse{ id: response.id})
        } else {
            Err(Error::from(ZulipApiError::FailedToPostMessage {message: response.msg}))
        }
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

    fn build_url(&self, path: &str) -> String {
        if self.zulip_domain.ends_with("/") {
            format!("{}{}", self.zulip_domain.as_str(), path)
        } else {
            format!("{}/{}", self.zulip_domain.as_str(), path)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalPostResponse {
    id: u32,
    msg: String,
    result: String
}
