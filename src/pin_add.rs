use crate::IpfsApi;
use std::error::Error;

impl IpfsApi {
    /// Publish an IPFS hash in IPNS.
    pub fn pin_add(&self, hash: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let endpoint = "api/v0/pin/add";
        let url = format!("http://{}:{}/{}", self.server, self.port, endpoint);

        let resp = ureq::post(&url)
            .query("arg", hash)
            .query("recursive", "true")
            .query("progress", "false")
            .call()?;

        let resp = resp.into_reader();
        let resp: serde_json::Value = serde_json::from_reader(resp)?;

        let mut pins = Vec::<String>::new();

        for val in resp["Pins"].as_array().ok_or("Pins is not an array")? {
            let str_val = val.as_str().ok_or("Value is not a string")?;
            pins.push(str_val.to_string());
        }

        Ok(pins)
    }
}
