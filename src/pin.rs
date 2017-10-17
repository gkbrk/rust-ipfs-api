use IpfsApi;

use std::collections::HashMap;
use reqwest;
use serde_json;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JsonDecode(serde_json::Error);
        UrlParseError(reqwest::UrlError);
    }
}

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all="PascalCase")]
pub struct PinAddResponse {
    pins: Vec<String>,
    progress: Option<u64>
}

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all="PascalCase")]
pub struct PinRmResponse {
    pins: Vec<String>
}

#[derive(Deserialize, Debug, PartialEq, Hash)]
pub struct PinType {
    #[serde(rename = "Type")]
    objtype: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct PinList {
    // keys: Vec<String>
    keys: HashMap<String, PinType>
}

impl IpfsApi {
    /// Tells the IPFS server to pin the given object.
    /// If 'recursive' is true, it will recursively pin all objects
    /// that one depends on.
    /// If 'progress' is true, it will return a percentage(?) progress
    /// if the object has not been already pinned, or None if it has.
    pub fn pin_add(&self, hash: &str, recursive: bool, progress: bool) -> Result<PinAddResponse> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/add");
        url.query_pairs_mut()
            .append_pair("arg", hash)
            .append_pair("recursive", &recursive.to_string())
            .append_pair("progress", &progress.to_string());
        let resp = reqwest::get(url)?;
        Ok(serde_json::from_reader(resp)?)
    }

    /// Unpin the given object.
    pub fn pin_rm(&self, hash: &str, recursive: bool) -> Result<PinRmResponse> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/rm");
        url.query_pairs_mut()
            .append_pair("arg", hash)
            .append_pair("recursive", &recursive.to_string());
        let resp = reqwest::get(url)?;
        Ok(serde_json::from_reader(resp)?)
    }


    /// List pinned objects.
    pub fn pin_ls(&self) -> Result<PinList> {
        let mut url = self.get_url()?;
        url.set_path("api/v0/pin/ls");
        // url.query_pairs_mut()
        //     .append_pair("arg", hash);
        //     .append_pair("recursive", &recursive.to_string());
        let resp = reqwest::get(url)?;
        Ok(serde_json::from_reader(resp)?)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use IpfsApi;
    use super::*;

    /// Add a pin,  list itthen remove it.
    ///
    /// This seems to alternately succeed and fail;
    /// half the time when you try to add a pi it doesn't
    /// return a correct value for the list of pins???
    ///
    /// Here's what it returns if the object is not pinned,
    /// apparently:
    /// > curl "http://localhost:5001/api/v0/pin/add?arg=QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u&recursive=true&progress=true"
    /// {"Pins":null,"Progress":1}
    /// {"Pins":["QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u"]}
    ///
    /// No idea how to make reqwest make something sensible of this.
    #[test]
    fn test_pin_full() {
        // Hello world object
        let obj = "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u";

        // Add pin
        let api = IpfsApi::new("127.0.0.1", 5001);
        let resp = api.pin_add(obj, true, true);
        // println!("Add response: {:#?}", resp);
        let desired = PinAddResponse {
            pins: vec![obj.into()],
            progress: None,
        };
        assert_eq!(resp.unwrap(), desired);



        // List pin to make sure it's present.
        let api = IpfsApi::new("127.0.0.1", 5001);
        let resp = api.pin_ls();
        // println!("Ls response: {:#?}", resp);
        assert!(resp.unwrap().keys.contains_key(obj));
        // assert_eq!(resp.unwrap(), desired);

        // Remove pin
        let api = IpfsApi::new("127.0.0.1", 5001);
        let resp = api.pin_rm(obj, true);
        // println!("Rm response: {:#?}", resp);
        let desired = PinRmResponse {
            pins: vec![obj.into()],
        };
        assert_eq!(resp.unwrap(), desired);
    }
}
