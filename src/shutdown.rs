use IpfsApi;

use reqwest;
use failure::Error;

impl IpfsApi {
    /// Shut down the IPFS daemon
    /// This function causes the IPFS daemon to terminate
    pub fn shutdown(&self) -> Result<(), Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/shutdown");
        let client = reqwest::Client::new();
        let _resp = client.post(url).send()?;
        Ok(())
    }
}
