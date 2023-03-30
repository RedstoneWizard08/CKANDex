use git2::Repository;
use serde::{Deserialize, Serialize};

use std::{
    fs::{read_to_string, write},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    schemas::{frozen::FrozenSchema, netkan::NetKANSchema},
    CKANError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestKind {
    NETKAN,
    FROZEN,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheJSON<T> {
    commit: String,
    data: T,
}

impl<T> CacheJSON<T> {
    pub fn from_data(commit: String, data: T) -> Self {
        return Self { commit, data };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheClient {
    pub netkans: Option<Vec<NetKANSchema>>,
    pub frozen: Option<Vec<FrozenSchema>>,
    pub dir: String,
}

impl Default for CacheClient {
    fn default() -> Self {
        return Self {
            netkans: None,
            frozen: None,
            dir: "".to_string(),
        };
    }
}

impl CacheClient {
    pub fn new(dir: String) -> Self {
        return Self {
            netkans: None,
            frozen: None,
            dir,
        };
    }

    pub fn is_netkan_cache_valid(&self) -> bool {
        let dir = PathBuf::from_str(&self.dir).unwrap();
        let repo = Repository::open(dir.clone()).unwrap();
        let commit = repo.revparse_single("HEAD").unwrap().id().to_string();

        let file_path = dir.join("netkan.cache.json");

        if !file_path.exists() {
            return false;
        }

        let file_data = read_to_string(file_path).unwrap();

        let data = serde_json::from_str::<CacheJSON<Vec<NetKANSchema>>>(&file_data).unwrap();

        if data.commit == commit {
            return true;
        }

        return false;
    }

    pub fn is_frozen_cache_valid(&self) -> bool {
        let dir = PathBuf::from_str(&self.dir).unwrap();
        let repo = Repository::open(dir.clone()).unwrap();
        let commit = repo.revparse_single("HEAD").unwrap().id().to_string();

        let file_path = dir.join("frozen.cache.json");

        if !file_path.exists() {
            return false;
        }

        let file_data = read_to_string(file_path).unwrap();

        let data = serde_json::from_str::<CacheJSON<Vec<FrozenSchema>>>(&file_data).unwrap();

        if data.commit == commit {
            return true;
        }

        return false;
    }

    pub fn is_cache_valid(&self) -> Result<bool, CKANError> {
        let netkan = self.is_netkan_cache_valid();
        let frozen = self.is_frozen_cache_valid();

        return Ok(netkan && frozen);
    }

    pub async fn update_cache(&mut self) -> Result<(), CKANError> {
        self.update_netkan_cache()?;
        self.update_frozen_cache()?;

        return Ok(());
    }

    /// Updates the NetKAN cache in the cache client.
    pub fn update_netkan_cache(&mut self) -> Result<(), CKANError> {
        // Check for valid cache
        if self.is_netkan_cache_valid() && self.netkans.clone().is_some() {
            return Ok(());
        }

        // Retrieve the commit hash.
        let dir = PathBuf::from_str(&self.dir).unwrap();
        let repo = Repository::open(dir.clone()).unwrap();
        let commit = repo.revparse_single("HEAD").unwrap().id().to_string();

        // Get the NetKAN data dir.
        let nk_dir = dir.join("NetKAN");
        let found = nk_dir.read_dir().unwrap();

        // Filter files for NetKAN files.
        let netkans: Vec<String> = found
            .map(|v| v.unwrap())
            .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
            .filter(|v| v.ends_with(".netkan"))
            .collect();

        // Make the list.
        let mut items: Vec<NetKANSchema> = Vec::new();

        // Parse and build the item list.
        for e in netkans {
            let read = read_to_string(e).unwrap();

            let json: NetKANSchema = match serde_json::from_str::<NetKANSchema>(&read) {
                Ok(val) => val,
                Err(_) => serde_yaml::from_str(&read).unwrap(),
            };

            items.push(json);
        }

        // Create the cache object.
        let cache_obj = CacheJSON::from_data(commit, items.clone());
        let file_path = dir.join("netkan.cache.json");

        // Write the cache object.
        write(file_path, serde_json::to_string(&cache_obj).unwrap()).unwrap();

        self.netkans = Some(items);

        return Ok(());
    }

    /// Updates the frozen cache in the cache client.
    pub fn update_frozen_cache(&mut self) -> Result<(), CKANError> {
        if self.is_frozen_cache_valid() && self.frozen.clone().is_some() {
            return Ok(());
        }

        // Retrieve the commit hash.
        let dir = PathBuf::from_str(&self.dir).unwrap();
        let repo = Repository::open(dir.clone()).unwrap();
        let commit = repo.revparse_single("HEAD").unwrap().id().to_string();

        // Get the NetKAN data dir.
        let nk_dir = dir.join("NetKAN");
        let found = nk_dir.read_dir().unwrap();

        // Filter files for frozen files.
        let frozens: Vec<String> = found
            .map(|v| v.unwrap())
            .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
            .filter(|v| v.ends_with(".frozen"))
            .collect();

        // Make the list.
        let mut items: Vec<FrozenSchema> = Vec::new();

        // Parse and build the item list.
        for e in frozens {
            let read = read_to_string(e).unwrap();

            if !read.contains(' ') {
                continue;
            }

            let json: FrozenSchema;

            match serde_json::from_str::<FrozenSchema>(&read) {
                Ok(val) => json = val,
                Err(_) => match serde_yaml::from_str::<FrozenSchema>(&read) {
                    Ok(v) => json = v,
                    Err(_) => continue,
                },
            };

            items.push(json);
        }

        // Create the cache object.
        let cache_obj = CacheJSON::from_data(commit, items.clone());
        let file_path = dir.join("frozen.cache.json");

        // Write the cache object.
        write(file_path, serde_json::to_string(&cache_obj).unwrap()).unwrap();

        self.frozen = Some(items);

        return Ok(());
    }
}
