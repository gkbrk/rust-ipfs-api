use IpfsApi;

use reqwest;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParseError(reqwest::UrlError);
    }
}

impl IpfsApi {
    /// Shut down the IPFS daemon
    /// This function causes the IPFS daemon to terminate
    pub fn shutdown(&self) -> Result<()> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/shutdown");
        let _resp = reqwest::get(url)?;
        Ok(())
    }
}
