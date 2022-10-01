use crate::IpfsApi;
use serde_json;
use std::error::Error;
use ureq;

pub struct IpfsVersion {
    val: serde_json::Value,
}

macro_rules! version_method_str {
    ($x:ident, $y:ident) => {
        #[doc = concat!("Returns the ", stringify!($y), " field of the IPFS version.")]
        #[must_use]
        pub fn $x(&self) -> Option<&str> {
            self.val[stringify!($y)].as_str()
        }
    };
}

impl IpfsVersion {
    version_method_str!(commit, Commit);
    version_method_str!(golang, Golang);
    version_method_str!(repo, Repo);
    version_method_str!(system, System);
    version_method_str!(version, Version);
}

impl IpfsApi {
    /// Get the version from the IPFS daemon.
    pub fn version(&self) -> Result<IpfsVersion, Box<dyn Error>> {
        let url =
            format!("http://{}:{}/api/v0/version", self.server, self.port);
        let resp = ureq::post(&url).call()?;
        let json_val = serde_json::from_reader(resp.into_reader())?;
        Ok(IpfsVersion { val: json_val })
    }
}
