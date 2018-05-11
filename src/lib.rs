//! # ipfsapi
//! This is a crate for interfacing with the local IPFS API. It allows you to
//! read and write data to the IPFS network.

extern crate reqwest;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate failure;
extern crate base64;

mod cat;
mod ipns;
mod object;
pub mod pin;
pub mod pubsub;
mod version;
mod shutdown;
mod log;
mod block;

pub struct IpfsApi {
    server: String,
    port: u16
}

/// The main interface of the library
/// The IpfsApi class represents a connection to the local IPFS daemon. It can
/// read and write data to it using http requests to the server.
impl IpfsApi {
    /// Creates a new instance of the API
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// ```
    pub fn new(server: &str, port: u16) -> IpfsApi {
        IpfsApi {
            server: server.into(),
            port: port
        }
    }

    /// Returns a Reqwest URL for the server
    /// Defaults to HTTP with no paths and no request parts.
    fn get_url(&self) -> Result<reqwest::Url, reqwest::UrlError> {
        let url_string = format!("http://{}:{}/", self.server, self.port);
        reqwest::Url::parse(&url_string)
    }
}

