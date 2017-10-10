use IpfsApi;

use serde_json::Value;

extern crate reqwest;
extern crate serde_json;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JsonDecode(serde_json::Error);
    }
}

impl IpfsApi {
    /// Resolve an IPNS hash or a domain name
    ///
    /// ```
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// let hash = api.name_resolve("gkbrk.com");
    ///
    /// println!("{}", hash);
    /// ```
    pub fn name_resolve(&self, name: &str) -> Result<String> {
        let url = format!("http://{}:{}/api/v0/name/resolve?arg={}", self.server, self.port, name);
        let resp = reqwest::get(&url)?;
        let resp: Value = serde_json::from_reader(resp)?;
        Ok(resp["Path"].to_string())
    }

    /// Publish an IPFS hash in IPNS.
    pub fn name_publish(&self, hash: &str) -> Result<()> {
        let url = format!("http://{}:{}/api/v0/name/publish?arg={}", self.server, self.port, hash);
        let _resp = reqwest::get(&url)?;
        Ok(())
    }
}
