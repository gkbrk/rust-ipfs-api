use crate::IpfsApi;
use std::io::Read;
use ureq;

impl IpfsApi {
    /// Get a raw IPFS block.
    ///
    /// This is a wrapper for the /api/v0/block/get API.
    ///
    /// ```rust
    /// # use ipfsapi::IpfsApi;
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// let hash = "QmV8cfu6n4NT5xRr2AHdKxFMTZEJrA44qgrBCr739BN9Wb";
    /// let block = api.block_get(hash).unwrap();
    /// let string = String::from_utf8(block.collect()).unwrap();
    ///
    /// assert_eq!(string, "Hello world");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn block_get(&self, hash: &str) -> Result<impl Iterator<Item = u8>, ureq::Error> {
        let url = format!("http://{}:{}/api/v0/block/get", self.server, self.port);
        let req = ureq::post(&url).query("arg", hash);
        let resp = req.call()?;
        let bytes = resp.into_reader().bytes();

        Ok(bytes.filter(|x| x.is_ok()).map(|x| x.unwrap()))
    }
}
