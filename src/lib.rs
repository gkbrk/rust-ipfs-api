#![feature(conservative_impl_trait)]

extern crate reqwest;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::Read;
use std::io::{BufReader, BufRead};

pub struct IpfsApi {
    server: String,
    port: u16
}

#[derive(Debug, Deserialize)]
pub struct PubSubMessage {
    pub data: Option<String>,
    pub from: Option<String>,
    pub seqno: Option<String>
}

impl IpfsApi {
    pub fn new(server: &str, port: u16) -> IpfsApi {
        IpfsApi {
            server: server.into(),
            port: port
        }
    }

    pub fn cat(&self, hash: &str) -> Result<impl Iterator<Item=u8>, reqwest::Error> {
        let url = format!("http://{}:{}/api/v0/cat?arg={}", self.server, self.port, hash);
        let resp = reqwest::get(&url)?;
        Ok(resp.bytes().filter(|x|x.is_ok()).map(|x|x.unwrap()))
    }

    pub fn pubsub_subscribe(&self, channel: &str) -> Result<impl Iterator<Item=PubSubMessage>, reqwest::Error> {
        let url = format!("http://{}:{}/api/v0/pubsub/sub?arg={}&discover=true", self.server, self.port, channel);
        let resp = reqwest::get(&url)?;

        let messages = BufReader::new(resp).lines()
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap())
            .map(|x|serde_json::from_str(&x))
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap());

        Ok(messages)
    }

    pub fn pubsub_publish(&self, channel: &str, data: &str) -> Result<(), reqwest::Error> {
        let url = format!("http://{}:{}/api/v0/pubsub/pub?arg={}&arg={}", self.server, self.port, channel, data);
        let resp = reqwest::get(&url)?;
        Ok(())
    }
}
