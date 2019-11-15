use crate::api::message::{MessagesResponse, PostResponse, Message, MessageEvent};
use crate::api::narrow::Narrow;
use crate::api::config::ZulipConfig;

use serde::{Deserialize, Serialize};
use failure::Error;
use reqwest::{Client};
use crate::api::errors::ZulipApiError;
use crate::api::queue_api::Queue;
use crate::api::errors::ZulipApiError::ZulipError;
use std::collections::HashMap;

#[derive(Debug)]
pub struct API {
    zulip_domain: String,
    user: String,
    pass: String,
    client: Client,
}

impl API {
    pub fn get_messages(
        &self,
        num_before: u32,
        num_after: u32,
        anchor: Option<i64>,
        narrows: &[Narrow],
    ) -> Result<MessagesResponse, Error> {
        let anchor_parameter = anchor
            .map(|a| ("anchor".to_string(), format!("{}", a)))
            .unwrap_or(("use_first_unread_anchor".to_string(), "true".to_string()));

        let url = self.build_url("api/v1/messages");

        let response: InternalMessagesResponse = self
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

        if response.result == "success" {
            Ok(MessagesResponse{
                messages: response.messages.unwrap_or_default(),
                anchor: response.anchor,
                found_anchor: response.found_anchor,
                found_newest: response.found_newest,
                found_oldest: response.found_oldest
            })
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }
    }

    pub fn post_message(&self, stream: &str, topic: &str, message: &str) -> Result<PostResponse, Error> {
        let url = self.build_url("api/v1/messages");
        let response: InternalPostResponse = self.client
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
            .json()?;

        if response.result == "success" {
            Ok(PostResponse{ id: response.id})
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }
    }

    pub fn mute(&self, stream: &str, topic: &str) -> Result<(), Error> {
        let response : InternalMuteResponse= self.client.patch(self.build_url("api/v1/users/me/subscriptions/muted_topics").as_str())
            .basic_auth(self.user.as_str(), Some(self.pass.as_str()))
            .body([
                format!("stream={}", stream),
                format!("topic={}", topic),
                "op=add".to_string()
            ].join("&")).send()?
            .json()?;
        if response.result == "success" {
            Ok(())
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }
    }

    pub fn create_queue(&self, all_public_streams: bool, narrows: &[Narrow]) -> Result<Queue, Error> {
        let url = self.build_url("api/v1/register");
        let response: InternalRegisterQueueResponse = self.client
            .post(url.as_str()).header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(self.user.as_str(), Some(self.pass.as_str()))
            .body([
                "event_types=[\"message\"]".to_string(),
                if all_public_streams {
                    "all_public_streams=true".to_string()
                } else {
                    "all_public_streams=false".to_string()
                },
                serde_json::to_string(narrows)?
            ].join("&")).send()?.json()?;

        if response.result == "success" {
            Ok(Queue{
                id: response.queue_id,
                last_event_id: response.last_event_id
            })
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }
    }

    pub fn get_queued_messages(&self, queue: &mut Queue) -> Result<Vec<Message>, Error> {
        let url = self.build_url("api/v1/events");

        let response: MessageQueueResponse = self.client
            .get(url.as_str())
            .basic_auth(self.user.as_str(), Some(self.pass.as_str()))
            .query(&[
                ("queue_id", queue.id.as_str()),
                ("last_event_id", format!("{}", queue.last_event_id).as_str())
            ]).send()?
            .json()?;

        let highest = response.events.iter().map(|e| e.id).max().unwrap_or(queue.last_event_id);
        queue.last_event_id = highest;
        if response.result == "success" {
            Ok(response.events.into_iter().map(|e| e.message).collect())
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }
    }

    pub fn create_stream(&self, name: &str, announce: bool) -> Result<i64, Error> {
        let url = self.build_url("/api/v1/users/me/subscriptions");


        let mut form = HashMap::new();
        form.insert("subscriptions", format!("[{{\"name\":\"{}\",\"description\":\"\"}}]", name));
        form.insert("announce", format!("{}", announce));

        let response: InternalCreateStreamResponse = self.client
            .post(url.as_str())
            .basic_auth(self.user.as_str(), Some(self.pass.as_str()))
            .form(&form)
            .send()?
            .json()?;

        if response.result == "success" {
            let stream_id = self.get_stream_id(name)?;
            Ok(stream_id)
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }

    }

    pub fn get_stream_id(&self, stream: &str) -> Result<i64, Error> {
        let url = self.build_url("/api/v1/get_stream_id");

        let response: InternalGetStreamIdResponse = self.client.get(url.as_str()).basic_auth(self.user.as_str(), Some(self.pass.as_str())).query(&[
            ("stream", stream)
        ]).send()?.json()?;

        if response.result == "success" && response.stream_id.is_some() {
            Ok(response.stream_id.unwrap())
        } else {
            Err(Error::from(ZulipApiError::ZulipError {message: response.msg}))
        }
    }

    pub fn delete_stream(&self, stream_id: i64) -> Result<(), Error> {
        let url = format!("{}/{}", self.build_url("/api/v1/streams/"), stream_id);

        let response: InternalDeleteStreamResponse = self.client.delete(url.as_str()).basic_auth(self.user.as_str(), Some(self.pass.as_str())).send()?.json()?;

        if response.result == "success" {
            Ok(())
        } else {
            Err(Error::from(ZulipError {message: response.msg}))
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
    id: i64,
    msg: String,
    result: String
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalMuteResponse {
    msg: String,
    result: String
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalRegisterQueueResponse {
    last_event_id: i64,
    msg: String,
    queue_id: String,
    result: String
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageQueueResponse {
    result: String,
    msg: String,
    events: Vec<MessageEvent>
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalMessagesResponse {
    result: String,
    msg: String,
    anchor: Option<i64>,
    found_newest: Option<bool>,
    found_oldest: Option<bool>,
    found_anchor: Option<bool>,
    messages: Option<Vec<Message>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalCreateStreamResponse {
    result: String,
    msg: String
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalGetStreamIdResponse {
    result: String,
    msg: String,
    stream_id: Option<i64>
}

#[derive(Debug, Serialize, Deserialize)]
struct InternalDeleteStreamResponse {
    result: String,
    msg: String
}
