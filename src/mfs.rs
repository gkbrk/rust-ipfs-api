use IpfsApi;

use std::path::PathBuf;
use std::collections::HashMap;
use reqwest;
use serde_json;
use failure;
use std::error;
use std::fmt;
use serde::{Deserializer, Deserialize};

pub type Fallible<T> = Result<T, failure::Error>;

trait PathBufNormalize {
    /// Normally, PathBuf won't convert a/b/../c to a/c in case b is a symlink. PathBuf is a little too smart for us. No symlinks on mfs.
    fn normalize_parents(&self) -> Self;
}

impl PathBufNormalize for PathBuf {
    fn normalize_parents(&self) -> Self {
        let mut norm = PathBuf::new();
        use std::path::Component::*;
        for comp in self.components() { match comp {
            RootDir=> norm = PathBuf::from("/"),
            CurDir => norm = PathBuf::from("."),
            Normal(dir) => norm.push(dir),
            ParentDir => { norm.pop(); },
            Prefix(_) => panic!("Windows Path Prefix detected in IPFS MFS path…"), // oO - I'm unsure how to construct a path from a prefix
        }}
        return norm
    }
}

pub struct MFS<'a> {
    pub api: &'a IpfsApi,
    cwd: PathBuf,
    flush: bool,
}

pub enum MfsNodeType {
    File, Folder
}
pub struct MfsNode {
    pub hash: String,
    pub size: u64,
    pub name: String,
    pub typ: MfsNodeType,
}
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MfsNodeIpfs {
    Hash: String,
    Size: u64,
    Name: String,
    Type: u64,
}
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FilesLsReply {
    #[serde(deserialize_with = "entries_deserialize")]
    Entries: Vec<MfsNodeIpfs>
}
fn entries_deserialize<'de, D>(deserializer: D) -> Result<Vec<MfsNodeIpfs>, D::Error>
where D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer) {
        Err(_) => Ok(vec![]), // TODO: I'd like to only do a vec![] if Entries was null
        ok => ok
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Stat {
    pub Hash: String,
    pub Size: u64,
    pub CumulativeSize: u64,
    pub Blocks: u64,
    pub Type: String, // TODO
}

// TODO: Move to is own module, use everywhere
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct IpfsError {
    Code: usize,
    Message: String,
    Type: String
}
impl error::Error for IpfsError {
    fn description(&self) -> &str {
        "Unknown MFS entry type"
    }
}
impl fmt::Display for IpfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ipfs API: {}", self.Message)
    }
}

// Not exactly my favourite, but I don't know what else to return - would be nice if serde_json
// could handle this entirely
#[derive(Debug)]
struct UnknownMfsEntryTypeError { 
    cause_type: u64,
}
impl error::Error for UnknownMfsEntryTypeError {
    fn description(&self) -> &str {
        "Unknown MFS entry type"
    }
}
impl fmt::Display for UnknownMfsEntryTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MFS type {} not known", self.cause_type)
    }
}


impl<'a> MFS<'a> {
    pub fn cwd(&self) -> &str { self.cwd.to_str().unwrap() }
    pub fn cd(&self, path: &str) -> Self {
        MFS {
            api: self.api,
            cwd: self.cwd.join(path).normalize_parents(),
            flush: self.flush,
        }
    }
    pub fn autoflush(&mut self, v: bool) -> &mut Self { self.flush = v; self }
    fn standardreq(&self, subpath: &str, args: Vec<(&str, &str)>) -> Fallible<reqwest::Response>
    {
        let mut url = self.api.get_url()?;
        url.set_path("api/v0/files/");
        let mut url = url.join(subpath)?;
        url.query_pairs_mut().extend_pairs(args).append_pair("arg", self.cwd());
        let resp = reqwest::get(url)?;
        if resp.status().is_success() {
            Ok(resp)
        } else {
            let err: IpfsError = serde_json::from_reader(resp)?;
            Err(err)?;
            panic!("unreachable") // TODO
        }
    }
    pub fn flush(&self) -> Fallible<()> { 
        self.standardreq("flush", vec![])?;
        Ok(())
    }
    pub fn mkdir(&self) -> Fallible<()> {
        self.standardreq("mkdir", vec![("parents", "true")])?;
        Ok(())
    }
    pub fn rm(&self) -> Fallible<()> {
        self.standardreq("rm", vec![("recursive", "true")])?;
        Ok(())
    }
    pub fn cp(&self, src: &str) -> Fallible<()> {
        self.standardreq("cp", vec![("arg", &("/ipfs/".to_string() + src))])?;
        Ok(())
    }
    /// cp will wail if the file already exists. This will not
    pub fn cpf(&self, src: &str) -> Fallible<()> {
        if let Err(_) = self.cp(src) {
            self.rm().ok();
            self.cp(src)?;
        }
        Ok(())
    }
    pub fn stat(&self) -> Fallible<Stat> {
        Ok(serde_json::from_reader(self.standardreq("stat", vec![])?)?)
    }
    pub fn ls(&self) -> Fallible<Vec<MfsNode>> {
        let resp = self.standardreq("ls", vec![("l", "true")])?;
        let resp: FilesLsReply = serde_json::from_reader(resp)?;
        resp.Entries.into_iter().map(|entry: MfsNodeIpfs| Ok(MfsNode {
            name: entry.Name,
            hash: entry.Hash,
            size: entry.Size,
            typ: (match entry.Type {
                0 => Ok(MfsNodeType::File),
                1 => Ok(MfsNodeType::Folder),
                n => Err(UnknownMfsEntryTypeError { cause_type: n })
            })?
        })).collect()
    }
    pub fn traverse<T>(&self, ap: &Fn(&PathBuf, &MfsNode) -> Option<T>) -> Fallible<HashMap<String, T>> {
        let mut ret = HashMap::new();
        self.traverse_impl(ap, &mut ret)?;
        Ok(ret)
    }
    pub fn traverse_impl<T>(&self, ap: &Fn(&PathBuf, &MfsNode) -> Option<T>, ret: &mut HashMap<String, T>) 
    -> Fallible<()> {
        for entry in self.ls()? {
            for t in ap(&self.cwd, &entry) {
                let mut path = self.cwd.clone();
                path.set_file_name(&entry.name);
                ret.insert(path.to_str().unwrap().to_string(), t);
            }
            match entry.typ {
                MfsNodeType::Folder => { self.cd(&entry.name).traverse_impl(ap, ret)?; },
                _ => {}
            };
        }
        Ok(())
    }
}

impl IpfsApi {
    pub fn mfs(&self) -> MFS {
        MFS {
            api: self,
            cwd: PathBuf::from("/"),
            flush: true
        }
    }
}
