//! # ipfsapi
//! This is a crate for interfacing with the local IPFS API. It allows you to
//! read and write data to the IPFS network.

mod pearson;
mod rand;
mod str_error;

mod cat;
mod shutdown;
pub mod version;

// API methods under /api/v0/block/.
mod block_get;
mod block_put;

// IPNS API
mod ipns_name_publish;
mod ipns_name_resolve;

// API methods under /api/v0/pin/.
mod pin_add;

//pub mod pin;
//pub mod pubsub;
//mod log;

pub struct IpfsApi {
    server: String,
    port: u16,
}

/// The main interface of the library
/// The `IpfsApi` class represents a connection to the local IPFS daemon. It can
/// read and write data to it using http requests to the server.
impl IpfsApi {
    /// Creates a new instance of the API
    ///
    /// ```rust
    /// # use ipfsapi::IpfsApi;
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// ```
    #[must_use]
    pub fn new(server: &str, port: u16) -> Self {
        Self {
            server: server.into(),
            port,
        }
    }
}
