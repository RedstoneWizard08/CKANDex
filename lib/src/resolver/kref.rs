use super::avc::AVCResolver;
use super::direct::DirectResolver;
use super::github::GitHubResolver;
use super::gitlab::GitLabResolver;
use super::jenkins::JenkinsResolver;
use super::netkan::NetKANResolver;
use super::common::ModResolver;

async fn end_resolve_kref(kref: String, token: String) -> Option<String> {
    let avc = AVCResolver::default();
    let github = GitHubResolver::default();
    let gitlab = GitLabResolver::default();
    let jenkins = JenkinsResolver::default();
    let direct = DirectResolver::default();

    if avc.should_resolve(kref.clone()) {
        return avc.resolve_url(kref, token).await;
    }

    if github.should_resolve(kref.clone()) {
        return github.resolve_url(kref, token).await;
    }

    if gitlab.should_resolve(kref.clone()) {
        return gitlab.resolve_url(kref, token).await;
    }

    if jenkins.should_resolve(kref.clone()) {
        return jenkins.resolve_url(kref, token).await;
    }

    if direct.should_resolve(kref.clone()) {
        return direct.resolve_url(kref, token).await;
    }

    return None;
}

pub async fn resolve_kref(kref: String, token: String) -> Option<String> {
    let netkan = NetKANResolver::default();

    if netkan.should_resolve(kref.clone()) {
        return end_resolve_kref(netkan.resolve_url(kref, token.clone()).await?, token).await;
    }

    return end_resolve_kref(kref.clone(), token).await;
}
