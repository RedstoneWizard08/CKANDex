use jenkins_api::JenkinsBuilder;

use crate::CKANError;

use super::common::{ModResolver, ModSourceLists};

#[derive(Default, Debug, Clone)]
pub struct JenkinsResolver {
    pub mods: ModSourceLists,
}

#[async_trait]
impl ModResolver for JenkinsResolver {
    fn should_resolve(&self, kref: String) -> bool {
        kref.starts_with("#/ckan/jenkins/")
    }

    async fn resolve_url(&self, kref: String, _: String) -> Result<String, CKANError> {
        let kref_url = kref.replace("#/ckan/jenkins/", "");
        let mut split_url = kref_url.split("/job/");

        let jenkins_url = split_url.next().unwrap();
        let mut job_name = split_url.next().unwrap().to_string();

        if job_name.ends_with('/') {
            let mut chars = job_name.chars();

            chars.next_back();

            let collected = chars.collect::<String>();

            job_name = collected;
        }

        let jenkins = JenkinsBuilder::new(jenkins_url).build().unwrap();
        let job = jenkins.get_job(&job_name).unwrap();

        let build = job
            .last_build
            .as_ref()
            .unwrap()
            .get_full_build(&jenkins)
            .unwrap();

        let artifact = build
            .artifacts
            .iter()
            .find(|a| a.file_name.ends_with(".zip"));

        if let Some(af) = artifact {
            let af_url = format!(
                "{}/lastSuccessfulBuild/artifact/{}",
                kref_url, af.relative_path
            );

            return Ok(af_url);
        }

        return Err(CKANError::UnknownArtifact);
    }

    fn merge_results(&self, other: &mut dyn ModResolver) {
        other.accept_mods(self.mods.clone());
    }

    fn accept_mods(&mut self, mods: ModSourceLists) {
        mods.avc.iter().for_each(|(k, v)| {
            self.mods.avc.insert(k.clone(), v.clone()).unwrap();
        });

        mods.spacedock.iter().for_each(|(k, v)| {
            self.mods.spacedock.insert(k.clone(), v.clone()).unwrap();
        });

        mods.github.iter().for_each(|(k, v)| {
            self.mods.github.insert(k.clone(), v.clone()).unwrap();
        });

        mods.gitlab.iter().for_each(|(k, v)| {
            self.mods.gitlab.insert(k.clone(), v.clone()).unwrap();
        });

        mods.netkan.iter().for_each(|(k, v)| {
            self.mods.netkan.insert(k.clone(), v.clone()).unwrap();
        });

        mods.direct.iter().for_each(|(k, v)| {
            self.mods.direct.insert(k.clone(), v.clone()).unwrap();
        });

        mods.jenkins.iter().for_each(|(k, v)| {
            self.mods.jenkins.insert(k.clone(), v.clone()).unwrap();
        });
    }
}
