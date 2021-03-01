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
pub struct Branch {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "internalName", skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "unspecified", skip_serializing_if = "Option::is_none")]
    pub unspecified: Option<bool>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "lastActivity", skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<String>,
    #[serde(rename = "groupFlag", skip_serializing_if = "Option::is_none")]
    pub group_flag: Option<bool>,
    #[serde(rename = "builds", skip_serializing_if = "Option::is_none")]
    pub builds: Option<crate::models::Builds>,
}

impl Branch {
    pub fn new() -> Branch {
        Branch {
            name: None,
            internal_name: None,
            default: None,
            unspecified: None,
            active: None,
            last_activity: None,
            group_flag: None,
            builds: None,
        }
    }
}


