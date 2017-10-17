use IpfsApi;

use reqwest;
use serde_json;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JsonDecode(serde_json::Error);
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct IpfsVersion {
    version: String,
    commit: String,
    repo: String,
    system: String,
    golang: String
}

impl IpfsApi {
    /// Get the version from the IPFS daemon.
    pub fn version(&self) -> Result<IpfsVersion> {
        let url = format!("http://{}:{}/api/v0/version", self.server, self.port);
        let resp = reqwest::get(&url)?;
        Ok(serde_json::from_reader(resp)?)
    }
}
