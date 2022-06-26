use crate::IpfsApi;
use ureq;

impl IpfsApi {
    /// Shut down the IPFS daemon
    /// This function causes the IPFS daemon to terminate
    pub fn shutdown(&self) -> Result<(), ureq::Error> {
        let url = format!("http://{}:{}/api/v0/shutdown", self.server, self.port);
        ureq::post(&url).call()?;
        Ok(())
    }
}
