use crate::IpfsApi;
use std::io::Read;
use ureq;

impl IpfsApi {
    /// Retrieves the contents of a file from the IPFS network. Takes a
    /// hash and returns an iterator of bytes. The result can be streamed, if
    /// the file is large.
    ///
    /// ```rust
    /// # use ipfsapi::IpfsApi;
    /// let api = IpfsApi::new("127.0.0.1", 5001);
    ///
    /// let hello = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")?;
    /// let hello_string = String::from_utf8(hello.collect())?;
    /// println!("{}", hello_string);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn cat(&self, hash: &str) -> Result<impl Iterator<Item = u8>, ureq::Error> {
        let url = format!("http://{}:{}/api/v0/cat", self.server, self.port);
        let req = ureq::post(&url).query("arg", hash);
        let resp = req.call()?;
        let bytes = resp.into_reader().bytes();

        Ok(bytes.filter(|x| x.is_ok()).map(|x| x.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::IpfsApi;

    #[test]
    fn test_cat_bytes() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        // Hello world object
        let bytes = api
            .cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")
            .unwrap();

        assert_eq!(
            "Hello World\n".as_bytes().to_vec(),
            bytes.collect::<Vec<u8>>()
        );
    }

    #[test]
    fn test_cat_string() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        let bytes = api
            .cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")
            .unwrap();
        let string = String::from_utf8(bytes.collect()).unwrap();

        assert_eq!("Hello World\n", &string);
    }
}
