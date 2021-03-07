use crate::provider::configuration::load_provider_config;
use crate::provider::{Provider, ProviderType};
use async_trait::async_trait;
use openapi::apis::configuration::Configuration;
use openapi::apis::project_api;

use crate::model::project::Project;

pub struct TeamCity;

#[async_trait]
impl Provider for TeamCity {
    fn new() -> TeamCity {
        TeamCity
    }

    async fn projects(&self) -> Vec<Project> {
        let cfg = load_provider_config(ProviderType::TeamCity).unwrap();
        let mut c: Configuration = Configuration::new();
        c.bearer_access_token = cfg.api_token;
        let res = project_api::get_all_projects(&c, None, None).await.unwrap();
        let mut vec = vec![];
        let ps = res.project.unwrap();
        for p in ps {
            let name = p.name.unwrap();
            vec.push(Project { name: name });
        }
        vec
    }
}
