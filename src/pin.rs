use IpfsApi;

use reqwest;
use serde_json;
use failure::{Error, err_msg};

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all="PascalCase")]
pub struct PinResponse {
    pins: Vec<String>
}

#[derive(PartialEq)]
pub enum PinType {
    Direct,
    Indirect,
    Recursive
}

pub struct PinnedHash {
    pub hash: String,
    pub pin_type: PinType
}

impl IpfsApi {
    /// Tells the IPFS server to pin the given object.
    /// If 'recursive' is true, it will recursively pin all objects
    /// that one depends on.
    /// If 'progress' is true, it will return a percentage(?) progress
    /// if the object has not been already pinned, or None if it has.
    pub fn pin_add(&self, hash: &str, recursive: bool) -> Result<PinResponse, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/add");
        url.query_pairs_mut()
            .append_pair("arg", hash)
            .append_pair("recursive", &recursive.to_string())
            .append_pair("progress", "false");
        let client = reqwest::Client::new();
        let resp = client.post(url).send()?;
        Ok(serde_json::from_reader(resp)?)
    }

    /// Unpin the given object.
    pub fn pin_rm(&self, hash: &str, recursive: bool) -> Result<PinResponse, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/rm");
        url.query_pairs_mut()
            .append_pair("arg", hash)
            .append_pair("recursive", &recursive.to_string());
        let client = reqwest::Client::new();
        let resp = client.post(url).send()?;
        Ok(serde_json::from_reader(resp)?)
    }


    /// List pinned objects.
    pub fn pin_list(&self) -> Result<Vec<PinnedHash>, Error> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/ls");
        let client = reqwest::Client::new();
        let resp = client.post(url).send()?;
        let json_resp: serde_json::Value = serde_json::from_reader(resp)?;

        let mut hashes = Vec::new();

        let keys = json_resp.get("Keys").ok_or(err_msg(""))?.as_object().ok_or(err_msg(""))?;

        for (key, value) in keys.iter() {
            hashes.push(PinnedHash {
                hash: key.clone(),
                pin_type: match &value.get("Type").ok_or(err_msg(""))?.as_str().ok_or(err_msg(""))? {
                    &"direct" => PinType::Direct,
                    &"indirect" => PinType::Indirect,
                    &"recursive" => PinType::Recursive,
                    _ => PinType::Direct
                }
            });
        }
        
        Ok(hashes)
    }
}


#[cfg(test)]
mod tests {
    use IpfsApi;
    use pin::PinType;

    // Add a pin, list it and then remove it.
    #[test]
    fn test_pin_full() {
        let api = IpfsApi::new("127.0.0.1", 5001);
        
        // Hello world object
        let hello = "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u";

        // Unpin everything first
        for pin in api.pin_list().unwrap() {
            if pin.pin_type == PinType::Direct || pin.pin_type == PinType::Recursive {
                api.pin_rm(&pin.hash, true).unwrap();
            }
        }

        // Add pin
        let resp = api.pin_add(hello, false).unwrap();
        
        // Check if pin is added
        assert_eq!(resp.pins.len(), 1);
        assert_eq!(resp.pins[0], "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u".to_string());
    }
}
