use crate::IpfsApi;
use std::error::Error;
use std::io::Cursor;
use std::io::Write;

impl IpfsApi {
    /// Puts a raw block into IPFS.
    ///
    /// # Notes
    /// This is a wrapper for the `/api/v0/block/put` API.
    ///
    pub fn block_put(&self, data: &[u8]) -> Result<String, Box<dyn Error>> {
        crate::rand::feed_event("block_put");
        let endpoint = "api/v0/block/put";
        let url = format!("http://{}:{}/{}", self.server, self.port, endpoint);

        let boundary = {
            let mut s = String::with_capacity(32);
            for _ in 0..32 {
                s.push(crate::rand::hex_digit());
            }
            s
        };

        let mut buffer: Vec<u8> = Vec::new();
        write!(buffer, "--{}\r\n", boundary)?;
        write!(buffer, "Content-Disposition: form-data; name=\"data\"\r\n")?;
        write!(buffer, "\r\n")?;
        buffer.extend(data);
        write!(buffer, "\r\n--{}--\r\n", boundary)?;

        let resp = ureq::post(&url)
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary=\"{}\"", boundary),
            )
            .send(Cursor::new(buffer))?;

        let resp = resp.into_reader();
        let resp: serde_json::Value = serde_json::from_reader(resp)?;

        Ok(resp["Key"].as_str().ok_or("Key not a string")?.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::IpfsApi;

    #[test]
    fn test_block_put() {
        let api = IpfsApi::new("127.0.0.1", 5001);
        let hash = api.block_put(b"Hello world").unwrap();
        let got: Vec<u8> = api.block_get(&hash).unwrap().collect();
        assert_eq!(got, b"Hello world");
    }
}
