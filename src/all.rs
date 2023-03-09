use std::{env::current_dir, fs::{read_to_string, write}, error::Error};
use git2::Repository;
use serde::{Serialize, Deserialize};

use crate::schemas::{netkan::NetKANSchema, frozen::FrozenSchema};

pub enum RequestKind {
    NETKAN,
    FROZEN,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheJSON<T> {
    commit: String,
    data: T,
}

impl<T> CacheJSON<T> {
    pub fn from_data(commit: String, data: T) -> Self {
        return Self {
            commit,
            data,
        };
    }
}

pub async fn get_cached_netkan() -> Result<Vec<NetKANSchema>, Box<dyn Error>> {
    let dir = current_dir()?.join("netkan");
    let repo = Repository::open(dir)?;
    let commit = repo.revparse_single("HEAD")?.id().to_string();

    let file_path = current_dir()?.join("netkan.cache.json");
    let file_data = read_to_string(file_path)?;

    let data = serde_json::from_str::<CacheJSON<Vec<NetKANSchema>>>(&file_data)?;

    if data.commit == commit {
        return Ok(data.data);
    }

    return Err("Invalid commit!".into());
}

pub async fn get_cached_frozen() -> Result<Vec<FrozenSchema>, Box<dyn Error>> {
    let dir = current_dir()?.join("netkan");
    let repo = Repository::open(dir)?;
    let commit = repo.revparse_single("HEAD")?.id().to_string();

    let file_path = current_dir()?.join("frozen.cache.json");
    let file_data = read_to_string(file_path)?;

    let data = serde_json::from_str::<CacheJSON<Vec<FrozenSchema>>>(&file_data)?;

    if data.commit == commit {
        return Ok(data.data);
    }

    return Err("Invalid commit!".into());
}

pub async fn get_netkans() -> Vec<NetKANSchema> {
    let cached = get_cached_netkan().await;

    if let Ok(val) = cached {
        return val;
    }

    let dir = current_dir().unwrap().join("netkan");
    let repo = Repository::open(dir.clone()).unwrap();
    let commit = repo.revparse_single("HEAD").unwrap().id().to_string();
    let nk_dir = dir.join("NetKAN");
    let found = nk_dir.read_dir().unwrap();

    let netkans: Vec<String> = found
        .map(|v| v.unwrap())
        .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
        .filter(|v| v.ends_with(".netkan"))
        .collect();

    let mut items: Vec<NetKANSchema> = Vec::new();

    for e in netkans {
        let read = read_to_string(e).unwrap();

        let json: NetKANSchema;

        match serde_json::from_str::<NetKANSchema>(&read) {
            Ok(val) => json = val,
            Err(_) => json = serde_yaml::from_str(&read).unwrap(),
        };

        if let Some(kref) = json.clone().kref {
            if !kref.contains("spacedock") {
                items.push(json);
            }
        }
    }

    let cache_obj = CacheJSON::from_data(commit, items.clone());
    let file_path = current_dir().unwrap().join("netkan.cache.json");
    
    write(file_path, serde_json::to_string(&cache_obj).unwrap()).unwrap();

    return items;
}

pub async fn get_frozen() -> Vec<FrozenSchema> {
    let cached = get_cached_frozen().await;

    if let Ok(val) = cached {
        return val;
    }

    let dir = current_dir().unwrap().join("netkan");
    let repo = Repository::open(dir.clone()).unwrap();
    let commit = repo.revparse_single("HEAD").unwrap().id().to_string();
    let nk_dir = dir.join("NetKAN");
    let found = nk_dir.read_dir().unwrap();

    let frozens: Vec<String> = found
        .map(|v| v.unwrap())
        .map(|v| v.path().as_os_str().to_str().unwrap().to_string())
        .filter(|v| v.ends_with(".frozen"))
        .collect();

    let mut items: Vec<FrozenSchema> = Vec::new();

    for e in frozens {
        let read = read_to_string(e).unwrap();

        if !read.contains(" ") {
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

        if let Some(kref) = json.clone().kref {
            if !kref.contains("spacedock") {
                items.push(json);
            }
        }
    }

    let cache_obj = CacheJSON::from_data(commit, items.clone());
    let file_path = current_dir().unwrap().join("frozen.cache.json");
    
    write(file_path, serde_json::to_string(&cache_obj).unwrap()).unwrap();

    return items;
}
