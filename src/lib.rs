#![feature(conservative_impl_trait)]

//! # ipfsapi
//! This is a crate for interfacing with the local IPFS API. It allows you to
//! read and write data to the IPFS network.

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

/// The main interface of the library
/// The IpfsApi class represents a connection to the local IPFS daemon. It can
/// read and write data to it using http requests to the server.
impl IpfsApi {
    /// Creates a new instance of the API
    ///
    /// ```
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// ```
    pub fn new(server: &str, port: u16) -> IpfsApi {
        IpfsApi {
            server: server.into(),
            port: port
        }
    }

    /// Retrieves the contents of a file from the IPFS network. Takes a
    /// hash and returns an iterator of bytes. The result can be streamed, if
    /// the file is large.
    ///
    /// ```
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// let hello = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")?;
    /// let hello_string = String::from_utf8(hello.collect())?;
    /// println!("{}", hello_string);
    /// ```
    pub fn cat(&self, hash: &str) -> Result<impl Iterator<Item=u8>, reqwest::Error> {
        let url = format!("http://{}:{}/api/v0/cat?arg={}", self.server, self.port, hash);
        let resp = reqwest::get(&url)?;
        Ok(resp.bytes().filter(|x|x.is_ok()).map(|x|x.unwrap()))
    }

    /// Subscribes to a channel for p2p messages
    /// This function subscribes to a channel/topic and listens to any data
    /// sent to it. It can be used for peer-to-peer communication and dynamic
    /// apps over IPFS.
    ///
    /// ```
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// for message in api.pubsub_subscribe("chat")? {
    ///     println!("{:?}", message);
    /// }
    /// ```
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

    /// Sends a p2p message to a channel
    /// This function sends a data packet to a channel/topic. It can be used
    /// for peer-to-peer communication and dynamic apps over IPFS.
    pub fn pubsub_publish(&self, channel: &str, data: &str) -> Result<(), reqwest::Error> {
        let url = format!("http://{}:{}/api/v0/pubsub/pub?arg={}&arg={}", self.server, self.port, channel, data);
        let resp = reqwest::get(&url)?;
        Ok(())
    }
}
