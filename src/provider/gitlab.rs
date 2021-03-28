use crate::model::project::Project;
use crate::provider::configuration::load_provider_config;
use crate::provider::{Pipeline, Provider, ProviderType};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;

pub struct GitLab;

#[async_trait]
impl Provider for GitLab {
    async fn projects(&self) -> Vec<Project> {
        let cfg = load_provider_config(ProviderType::GitLab).unwrap();
        let mut vec: Vec<Project> = vec![];
        let client = reqwest::Client::new();
        let r = client.get("https://gitlab.com/api/graphql")
                    .body("{\"query\":\"query {\\r\\n  projects(membership: true, search: \\\"\\\") {\\r\\n    nodes {\\r\\n      name\\r\\n    }\\r\\n    pageInfo {\\r\\n      endCursor\\r\\n      hasNextPage\\r\\n    }\\r\\n  }\\r\\n}\\r\\n\",\"variables\":{}}")
                    .bearer_auth(cfg.api_token.unwrap())
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .header(reqwest::header::ACCEPT, "application/json");

        let res = r.send().await.unwrap();
        let j: Value = res.json().await.unwrap();
        for n in j["data"]["projects"]["nodes"].as_array().unwrap() {
            vec.push(Project {
                name: n["name"].as_str().unwrap().to_string(),
            });
        }
        vec
    }

    async fn pipelines(&self) -> Vec<Pipeline> {
        vec![]
    }
}

impl GitLab {
    pub fn new() -> GitLab {
        GitLab
    }
}
