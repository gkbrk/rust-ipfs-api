use crate::str_error::StrError;
use crate::IpfsApi;
use serde_json::Value;
use std::error::Error;

impl IpfsApi {
    /// Resolve an IPNS hash or a domain name
    ///
    /// ```rust
    /// # use ipfsapi::IpfsApi;
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    /// let hash = api.name_resolve("gkbrk.com")?;
    ///
    /// println!("{}", hash);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn name_resolve(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let endpoint = "api/v0/name/resolve";
        let url = format!("http://{}:{}/{}", self.server, self.port, endpoint);
        let resp = ureq::post(&url).query("arg", name).call()?;
        let resp: Value = serde_json::from_reader(resp.into_reader())?;

        match &resp["Path"] {
            Value::String(x) => Ok(x.into()),
            _ => Err(StrError::from_str("Response does not contain 'Path'")),
        }
    }
}
