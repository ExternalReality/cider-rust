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
pub struct VcsStatus {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<crate::models::VcsCheckStatus>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<crate::models::VcsCheckStatus>,
}

impl VcsStatus {
    pub fn new() -> VcsStatus {
        VcsStatus {
            current: None,
            previous: None,
        }
    }
}


