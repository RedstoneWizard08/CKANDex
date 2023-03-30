use git2::{Repository, ResetType};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RepoInfo {
    pub url: &'static str,
    pub branch: &'static str,
}

pub static KSP1_REPO_INFO: RepoInfo = RepoInfo {
    url: "https://github.com/KSP-CKAN/NetKAN",
    branch: "master",
};

pub static KSP2_REPO_INFO: RepoInfo = RepoInfo {
    url: "https://github.com/KSP-CKAN/KSP2-NetKAN",
    branch: "main",
};

pub async fn update_repo(repo_info: RepoInfo, repo_dir: &str) {
    let dir = PathBuf::from_str(repo_dir).unwrap();

    let repo = match Repository::open(dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let commit = repo.revparse_single("HEAD").unwrap();

    repo.reset(&commit, ResetType::Hard, None).unwrap();

    repo.find_remote("origin")
        .unwrap()
        .fetch(&[repo_info.branch], None, None)
        .unwrap();

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

    if analysis.0.is_up_to_date() {
        return;
    } else if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", repo_info.branch);
        let mut reference = repo.find_reference(&refname).unwrap();

        reference
            .set_target(fetch_commit.id(), "Fast-Forward")
            .unwrap();

        repo.set_head(&refname).unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .unwrap();
    } else {
        panic!("Fast-forward only!");
    }
}

pub async fn clone_repo(repo_info: RepoInfo, repo_dir: &str) {
    let dir = PathBuf::from_str(repo_dir).unwrap();

    if dir.clone().exists() {
        update_repo(repo_info, repo_dir).await;

        return;
    }

    match Repository::clone_recurse(repo_info.url, dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    return;
}
