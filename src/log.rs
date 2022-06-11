use std::io::{BufReader, BufRead};

use IpfsApi;

use reqwest;
use serde_json;
use serde_json::Value;
use failure::Error;

impl IpfsApi {
    /// Returns an iterator to the log stream. Since the log entries are
    /// flexible, a dynamic Serde value is returned.
    pub fn log_tail(&self) -> Result<impl Iterator<Item=Value>, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/log/tail");
        let client = reqwest::Client::new();
        let resp = client.post(url).send()?;

        let messages = BufReader::new(resp).lines()
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap())
            .map(|x|serde_json::from_str(&x))
            .filter(|x|x.is_ok())
            .map(|x|x.unwrap());

        Ok(messages)
    }
}
