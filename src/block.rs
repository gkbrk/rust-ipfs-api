use std::io::Read;

#[derive(Deserialize)]
struct BlockPutResponse {
    #[serde(rename = "Key")]
    key: String,
}

impl IpfsApi {
    pub fn block_put<S: 'static +  Read + Send>(&self, data: S) -> Result<String, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/block/put");

        let mut res = {
            let client = reqwest::Client::new();
            client.post(url)
            .multipart(reqwest::multipart::Form::new()
                .part("arg", reqwest::multipart::Part::reader(data))
                ).send()?
        };

        let json: BlockPutResponse = res.json()?;
        Ok(json.key)
    }

    pub fn block_get(&self, hash: &str) -> Result<impl Iterator<Item=u8>, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/block/get");
        url.query_pairs_mut()
            .append_pair("arg", hash);

        let resp = reqwest::get(url)?;
        Ok(resp.bytes().filter(|x|x.is_ok()).map(|x|x.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use IpfsApi;

    #[test]
    fn test_block_put() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        let hash = api.block_put("Hello world".as_bytes()).unwrap();

        assert_eq!(hash, "QmV8cfu6n4NT5xRr2AHdKxFMTZEJrA44qgrBCr739BN9Wb");
    }

    #[test]
    fn test_block_get() {
        let api = IpfsApi::new("127.0.0.1", 5001);

        let content = "Hello world\n".as_bytes();

        let hash = api.block_put(content).unwrap();
        let block: Vec<u8> = api.block_get(&hash).unwrap().collect();

        assert_eq!(block, content);
    }
}
