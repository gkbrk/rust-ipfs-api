use crate::IpfsApi;
use std::error::Error;

impl IpfsApi {
    /// Publish an IPFS hash in IPNS.
    pub fn name_publish(&self, hash: &str) -> Result<(), Box<dyn Error>> {
        let endpoint = "api/v0/name/publish";
        let url = format!("http://{}:{}/{}", self.server, self.port, endpoint);
        ureq::post(&url).query("arg", hash).call()?;

        Ok(())
    }
}
