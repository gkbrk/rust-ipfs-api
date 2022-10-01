use crate::IpfsApi;
use std::io::Read;

impl IpfsApi {
    /// Get a raw IPFS block.
    ///
    /// # Notes
    /// This is a wrapper for the `/api/v0/block/get` API.
    ///
    /// # Errors
    /// This function can return an error if the IPFS node is down, or if the
    /// block cannot be retrieved.
    ///
    /// # Examples
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
    pub fn block_get(
        &self,
        hash: &str,
    ) -> Result<impl Iterator<Item = u8>, ureq::Error> {
        crate::rand::feed_event("block_get");

        let url =
            format!("http://{}:{}/api/v0/block/get", self.server, self.port);
        let req = ureq::post(&url).query("arg", hash);
        let resp = req.call()?;
        let bytes = resp.into_reader().bytes();

        Ok(bytes.filter_map(std::result::Result::ok))
    }
}
