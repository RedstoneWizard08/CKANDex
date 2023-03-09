use git2::{Repository, ResetType};
use std::env::current_dir;

pub static REPO: &str = "https://github.com/KSP-CKAN/NetKAN";
pub static NETKAN_BRANCH: &str = "master";

pub async fn update_repo() {
    let dir = current_dir().unwrap().join("netkan");

    let repo = match Repository::open(dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let commit = repo.revparse_single("HEAD").unwrap();

    repo.reset(
        &commit,
        ResetType::Hard,
        None,
    )
    .unwrap();

    repo.find_remote("origin")
        .unwrap()
        .fetch(&[NETKAN_BRANCH], None, None)
        .unwrap();

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

    if analysis.0.is_up_to_date() {
        return;
    } else if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", NETKAN_BRANCH);
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

pub async fn clone_repo() {
    let dir = current_dir().unwrap().join("netkan");

    if dir.clone().exists() {
        update_repo().await;

        return;
    }

    match Repository::clone_recurse(REPO, dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    return;
}
