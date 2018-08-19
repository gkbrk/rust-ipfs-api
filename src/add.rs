use IpfsApi;

use reqwest;
use reqwest::{Url, Client, RequestBuilder};
use failure::Error;
use failure::err_msg;
use std::collections::HashMap;
use serde_json;
use serde_json::Value;
use std::io::Read;

pub type Fallible<T> = Result<T, Error>;

pub struct IpfsAdd<'a> {
    api: &'a IpfsApi,
    settings: HashMap<String, String>,
}

impl<'a> IpfsAdd<'a> {
    // Nonsense: arg [file]: The path to a file to be added to ipfs. Required: yes.
    // recursive [bool]: Add directory paths recursively. Default: “false”. Required: no.
    pub fn recursive(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("recursive".to_string(), v.to_string()); self }
    // Nonsense: quiet [bool]: Write minimal output. Required: no.
    // Nonsense: quieter [bool]: Write only final hash. Required: no.
    // Nonsense: silent [bool]: Write no output. Required: no.
    // TODO: progress [bool]: Stream progress data. Required: no.
    //trickle [bool]: Use trickle-dag format for dag generation. Required: no.
    pub fn trickle(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("trickle".to_string(), v.to_string()); self }
    //only-hash [bool]: Only chunk and hash - do not write to disk. Required: no.
    pub fn only_hash(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("only-hash".to_string(), v.to_string()); self }
    //wrap-with-directory [bool]: Wrap files with a directory object. Required: no.
    pub fn wrap(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("wrap-with-directory".to_string(), v.to_string()); self }
    //hidden [bool]: Include files that are hidden. Only takes effect on recursive add. Required: no.
    pub fn hidden(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("hidden".to_string(), v.to_string()); self }
    //chunker [string]: Chunking algorithm to use. Required: no.
    pub fn chunker(&'a mut self, v: String) -> &'a mut IpfsAdd { self.settings.insert("chunker".to_string(), v); self }
    //pin [bool]: Pin this object when adding. Default: “true”. Required: no.
    pub fn pin(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("pin".to_string(), v.to_string()); self }
    //raw-leaves [bool]: Use raw blocks for leaf nodes. (experimental) -> &'a mut IpfsAdd. Required: no.
    pub fn raw_leaves(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("raw-leaves".to_string(), v.to_string()); self }
    //nocopy [bool]: Add the file using filestore. (experimental) -> &'a mut IpfsAdd. Required: no.
    pub fn nocopy(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("nocopy".to_string(), v.to_string()); self }
    //fscache [bool]: Check the filestore for pre-existing blocks. (experimental) -> &'a mut IpfsAdd. Required: no.
    pub fn fscache(&'a mut self, v: bool) -> &'a mut IpfsAdd { self.settings.insert("fscache".to_string(), v.to_string()); self }
    //cid-version [int]: Cid version. Non-zero value will change default of ‘raw-leaves’ to true. (experimental) -> &'a mut IpfsAdd. Default: “0”. Required: no.
    pub fn cid(&'a mut self, v: u32) -> &'a mut IpfsAdd { self.settings.insert("cid-version".to_string(), v.to_string()); self }
    //hash [string]: Hash function to use. Will set Cid version to 1 if used. (experimental) -> &'a mut IpfsAdd. Default: “sha2-256”. Required: no.
    pub fn hash(&'a mut self, v: String) -> &'a mut IpfsAdd { self.settings.insert("hash".to_string(), v); self }

    fn prep(&self) -> Fallible<(Client, Url)> {
        // I'd like to make prep and exec one function and have them call an FnOnce, but rust aint easy.
        let mut url = self.api.get_url()?;
        url.set_path("api/v0/add");
        for (k, v) in &self.settings {
            url.query_pairs_mut().append_pair(k, v);
        }
        let client = reqwest::Client::new();
        return Ok((client, url))
    }
    fn exec(&self, req: &mut RequestBuilder) -> Fallible<String> {
        let resp: Value = serde_json::from_reader(req.send()?)?;
        
        if resp["Hash"].is_string() {
            Ok(resp["Hash"].as_str().unwrap().into())
        } else {
            Err(err_msg("Key error"))
        }
    }

    pub fn read_from<S: 'static + Read + Send>(&self, data: S) -> Fallible<String> {
        let (client, url) = self.prep()?;
        self.exec(client.post(url)
            .multipart(reqwest::multipart::Form::new()
                .part("file", reqwest::multipart::Part::reader(data))))
    }
    pub fn from_path(&self, path: &str) -> Fallible<String> {
        let (client, mut url) = self.prep()?;
         url.query_pairs_mut().append_pair("file", path);
        self.exec(&mut client.get(url))
    }
}

impl IpfsApi {
    pub fn add(&self) -> IpfsAdd {
        IpfsAdd {
            api: &self,
            settings: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use IpfsApi;
}
