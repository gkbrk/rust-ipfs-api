#![feature(conservative_impl_trait)]

//! # ipfsapi
//! This is a crate for interfacing with the local IPFS API. It allows you to
//! read and write data to the IPFS network.

extern crate reqwest;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
extern crate base64;

use std::io::Read;

mod pin;
mod pubsub;
mod ipns;
mod object;
mod version;

#[derive(Clone, PartialEq, Hash, Debug)]
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

    /// Retrieves the contents of a file from the IPFS network. Takes a
    /// hash and returns an iterator of bytes. The result can be streamed, if
    /// the file is large.
    ///
    /// ```rust,ignore
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
}


#[cfg(test)]
mod tests {
    use IpfsApi;
    #[test]
    fn test_cat() {
        let api = IpfsApi::new("127.0.0.1", 5001);
        // Hello world object
        let bytes = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u").unwrap();
        let data = String::from_utf8(bytes.collect()).unwrap();

        assert_eq!("Hello World\n", &data);
    }
}