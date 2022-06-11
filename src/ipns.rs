use IpfsApi;

use serde_json::Value;

use reqwest;
use serde_json;
use failure::err_msg;
use failure::Error;

impl IpfsApi {
    /// Resolve an IPNS hash or a domain name
    ///
    /// ```rust,ignore
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// let hash = api.name_resolve("gkbrk.com");
    ///
    /// println!("{}", hash);
    /// ```
    pub fn name_resolve(&self, name: &str) -> Result<String, Error> {
        let url = format!("http://{}:{}/api/v0/name/resolve?arg={}", self.server, self.port, name);
        let client = reqwest::Client::new();
        let resp = client.post(&url).send()?;
        let resp: Value = serde_json::from_reader(resp)?;
        
        if resp["Path"].is_string() {
            Ok(resp["Path"].as_str().unwrap().into())
        } else {
            Err(err_msg("Key error"))
        }
    }

    /// Publish an IPFS hash in IPNS.
    pub fn name_publish(&self, hash: &str) -> Result<(), Error> {
        let url = format!("http://{}:{}/api/v0/name/publish?arg={}", self.server, self.port, hash);
        let client = reqwest::Client::new();
        let _resp = client.post(&url).send()?;
        Ok(())
    }
}
