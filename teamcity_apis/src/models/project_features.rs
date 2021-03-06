/*
 * TeamCity REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2018.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectFeatures {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "projectFeature", skip_serializing_if = "Option::is_none")]
    pub project_feature: Option<Vec<crate::models::ProjectFeature>>,
}

impl ProjectFeatures {
    pub fn new() -> ProjectFeatures {
        ProjectFeatures {
            count: None,
            href: None,
            project_feature: None,
        }
    }
}


