use git2::{Repository, ResetType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::CKANError;

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

pub async fn update_repo(repo_info: RepoInfo, dir: PathBuf) -> Result<(), CKANError> {
    let repo = Repository::open(dir)?;
    let commit = repo.revparse_single("HEAD")?;

    repo.reset(&commit, ResetType::Hard, None)?;

    repo.find_remote("origin")?
        .fetch(&[repo_info.branch], None, None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.0.is_up_to_date() {
        return Ok(());
    }

    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", repo_info.branch);
        let mut reference = repo.find_reference(&refname)?;

        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

        Ok(())
    } else {
        Err(CKANError::FastForwardOnly)
    }
}

pub async fn clone_repo(repo_info: RepoInfo, dir: PathBuf) -> Result<(), CKANError> {
    if dir.clone().exists() {
        update_repo(repo_info, dir).await?;

        return Ok(());
    }

    Repository::clone_recurse(repo_info.url, dir)?;

    Ok(())
}
