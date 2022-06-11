use std::io::{BufReader, BufRead};

use IpfsApi;

use reqwest;
use serde_json;
use base64;
use failure::Error;

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
    pub fn pubsub_subscribe(&self, channel: &str) -> Result<impl Iterator<Item=PubSubMessage>, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pubsub/sub");
        url.query_pairs_mut()
            .append_pair("arg", channel)
            .append_pair("discover", "true");
        let client = reqwest::Client::new();
        let resp = client.post(url).send()?;

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
    pub fn pubsub_publish(&self, channel: &str, data: &str) -> Result<(), Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pubsub/pub");
        url.query_pairs_mut()
            .append_pair("arg", channel)
            .append_pair("arg", data);
        let client = reqwest::Client::new();
        let _resp = client.post(url).send()?;
        Ok(())
    }
}
