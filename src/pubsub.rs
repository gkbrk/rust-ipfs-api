use std::io::{BufReader, BufRead};

use IpfsApi;

use reqwest;
use serde_json;
use base64;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JsonDecode(serde_json::Error);
    }
}

#[derive(Deserialize)]
struct JsonPubSubMessage {
    data: String,
    from: String,
    seqno: String
}

#[derive(Debug)]
pub struct PubSubMessage {
    data: Option<Vec<u8>>,
    from: Option<Vec<u8>>,
    seqno: Option<Vec<u8>>
}

impl PubSubMessage {
    pub fn data(&self) -> Option<Vec<u8>> {
        self.data.clone()
    }

    pub fn from(&self) -> Option<Vec<u8>> {
        self.from.clone()
    }

    pub fn seqno(&self) -> Option<Vec<u8>> {
        self.seqno.clone()
    }
}

impl IpfsApi {
    /// Subscribes to a channel for p2p messages
    /// This function subscribes to a channel/topic and listens to any data
    /// sent to it. It can be used for peer-to-peer communication and dynamic
    /// apps over IPFS.
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// for message in api.pubsub_subscribe("chat")? {
    ///     println!("{:?}", message);
    /// }
    /// ```
    pub fn pubsub_subscribe(&self, channel: &str) -> Result<impl Iterator<Item=PubSubMessage>> {
        let url = format!("http://{}:{}/api/v0/pubsub/sub?arg={}&discover=true", self.server, self.port, channel);
        let resp = reqwest::get(&url)?;

        let messages = BufReader::new(resp).lines()
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap())
            .map(|x|serde_json::from_str::<JsonPubSubMessage>(&x))
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap())
            .map(|x| {
                PubSubMessage {
                    from: base64::decode(&x.from).ok(),
                    seqno: base64::decode(&x.seqno).ok(),
                    data: base64::decode(&x.data).ok()
                }
            });

        Ok(messages)
    }
    
    /// Sends a p2p message to a channel
    /// This function sends a data packet to a channel/topic. It can be used
    /// for peer-to-peer communication and dynamic apps over IPFS.
    pub fn pubsub_publish(&self, channel: &str, data: &str) -> Result<()> {
        let url = format!("http://{}:{}/api/v0/pubsub/pub?arg={}&arg={}", self.server, self.port, channel, data);
        let _resp = reqwest::get(&url)?;
        Ok(())
    }
}
