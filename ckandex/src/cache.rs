use git2::Repository;
use serde::{Deserialize, Serialize};
use sha256::digest;

use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use crate::{
    clone_repo,
    schemas::{frozen::FrozenSchema, netkan::NetKANSchema},
    CKANError, KSP1_REPO_INFO, KSP2_REPO_INFO,
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
        Self { commit, data }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheClient {
    pub netkans: Option<Vec<NetKANSchema>>,
    pub frozen: Option<Vec<FrozenSchema>>,
    pub dir: PathBuf,
}

impl CacheClient {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            netkans: None,
            frozen: None,
            dir: dir.into(),
        }
    }

    pub async fn refresh(&self) {
        clone_repo(KSP1_REPO_INFO, self.dir.join("ksp1")).await;
        clone_repo(KSP2_REPO_INFO, self.dir.join("ksp2")).await;
    }

    pub fn is_netkan_cache_valid(&self) -> bool {
        let commit = self.commit();
        let file_path = self.dir.join("netkan.cache.json");

        if !file_path.exists() {
            return false;
        }

        let file_data = read_to_string(file_path).unwrap();
        let data = serde_json::from_str::<CacheJSON<Vec<NetKANSchema>>>(&file_data).unwrap();

        if data.commit == commit {
            return true;
        }

        false
    }

    pub fn is_frozen_cache_valid(&self) -> bool {
        let commit = self.commit();
        let file_path = self.dir.join("frozen.cache.json");

        if !file_path.exists() {
            return false;
        }

        let file_data = read_to_string(file_path).unwrap();
        let data = serde_json::from_str::<CacheJSON<Vec<FrozenSchema>>>(&file_data).unwrap();

        if data.commit == commit {
            return true;
        }

        false
    }

    pub fn is_cache_valid(&self) -> Result<bool, CKANError> {
        let netkan = self.is_netkan_cache_valid();
        let frozen = self.is_frozen_cache_valid();

        Ok(netkan && frozen)
    }

    pub async fn update_cache(&mut self) -> Result<(), CKANError> {
        self.update_netkan_cache()?;
        self.update_frozen_cache()?;

        Ok(())
    }

    pub fn commit(&self) -> String {
        let repo = Repository::open(self.dir.join("ksp1")).unwrap();
        let commit_ksp1 = repo.revparse_single("HEAD").unwrap().id().to_string();

        let repo = Repository::open(self.dir.join("ksp2")).unwrap();
        let commit_ksp2 = repo.revparse_single("HEAD").unwrap().id().to_string();

        digest(format!("{}+{}", commit_ksp1, commit_ksp2))
    }

    /// Updates the NetKAN cache in the cache client.
    pub fn update_netkan_cache(&mut self) -> Result<(), CKANError> {
        // Check for valid cache
        if self.is_netkan_cache_valid() && self.netkans.clone().is_some() {
            return Ok(());
        }

        // Retrieve the commit hash.
        let commit = self.commit();

        // Get the NetKAN data dir (KSP1).
        let nk_dir = self.dir.join("ksp1").join("NetKAN");
        let found = nk_dir.read_dir().unwrap();

        // Filter files for NetKAN files (KSP1).
        let mut netkans: Vec<String> = found
            .map(|v| v.unwrap())
            .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
            .filter(|v| v.ends_with(".netkan"))
            .collect();

        // Get the NetKAN data dir (KSP2).
        let nk_dir = self.dir.join("ksp2").join("NetKAN");
        let found = nk_dir.read_dir().unwrap();

        // Filter files for NetKAN files (KSP2).
        netkans.extend(
            found
                .map(|v| v.unwrap())
                .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
                .filter(|v| v.ends_with(".netkan"))
                .collect::<Vec<_>>(),
        );

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
        let file_path = self.dir.join("netkan.cache.json");

        // Write the cache object.
        write(file_path, serde_json::to_string(&cache_obj).unwrap()).unwrap();

        self.netkans = Some(items);

        Ok(())
    }

    /// Updates the frozen cache in the cache client.
    pub fn update_frozen_cache(&mut self) -> Result<(), CKANError> {
        if self.is_frozen_cache_valid() && self.frozen.clone().is_some() {
            return Ok(());
        }

        // Retrieve the commit hash.
        let commit = self.commit();

        // Get the NetKAN data dir.
        let nk_dir = self.dir.join("ksp1").join("NetKAN");
        let found = nk_dir.read_dir().unwrap();

        // Filter files for frozen files.
        let mut frozens: Vec<String> = found
            .map(|v| v.unwrap())
            .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
            .filter(|v| v.ends_with(".frozen"))
            .collect();

        // Get the NetKAN data dir (KSP2).
        let nk_dir = self.dir.join("ksp2").join("NetKAN");
        let found = nk_dir.read_dir().unwrap();

        // Filter files for NetKAN files (KSP2).
        frozens.extend(
            found
                .map(|v| v.unwrap())
                .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
                .filter(|v| v.ends_with(".frozen"))
                .collect::<Vec<_>>(),
        );

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
        let file_path = self.dir.join("frozen.cache.json");

        // Write the cache object.
        write(file_path, serde_json::to_string(&cache_obj).unwrap()).unwrap();

        self.frozen = Some(items);

        Ok(())
    }
}
