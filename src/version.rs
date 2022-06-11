use IpfsApi;

use reqwest;
use serde_json;
use failure::Error;

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
    pub fn version(&self) -> Result<IpfsVersion, Error> {
        let url = format!("http://{}:{}/api/v0/version", self.server, self.port);
        let client = reqwest::Client::new();
        let resp = client.post(&url).send()?;
        Ok(serde_json::from_reader(resp)?)
    }
}
