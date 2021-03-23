use crate::provider::configuration::load_provider_config;
use crate::provider::{Provider, ProviderType};
use async_trait::async_trait;
use openapi::apis::configuration::Configuration;
use openapi::apis::{project_api, build_type_api};

use crate::model::project::{Project, Pipeline};

pub struct TeamCity;

#[async_trait]
impl Provider for TeamCity {
    async fn projects(&self) -> Vec<Project> {
        let cfg = load_provider_config(ProviderType::TeamCity).unwrap();
        let mut c: Configuration = Configuration::new();
        c.bearer_access_token = cfg.api_token;
        let res = project_api::get_all_projects(&c, None, None).await.unwrap();
        let mut vec = vec![];
        let ps = res.project.unwrap();
        for p in ps {
            let name = p.name.unwrap();
            vec.push(Project { name });
        }
        vec
    }
    async fn pipelines(&self) -> Vec<Pipeline> {
        let cfg = load_provider_config(ProviderType::TeamCity).unwrap();
        let mut rc = Configuration::new();
        let mut vec : Vec<Pipeline> = vec!();
        rc.bearer_access_token = cfg.api_token;
        let m = build_type_api::get_all_build_types(&rc, None, None)
            .await
            .unwrap();
        let results = m.build_type.unwrap();
        for p in results {
            vec.push(Pipeline{
                name: p.name.unwrap(),
            });
        }
        vec
    }
}

impl TeamCity {
    pub fn new() -> TeamCity {
        TeamCity
    }     
}