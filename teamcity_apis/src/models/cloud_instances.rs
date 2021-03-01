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
pub struct CloudInstances {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "nextHref", skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
    #[serde(rename = "prevHref", skip_serializing_if = "Option::is_none")]
    pub prev_href: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "cloudInstance", skip_serializing_if = "Option::is_none")]
    pub cloud_instance: Option<Vec<crate::models::CloudInstance>>,
}

impl CloudInstances {
    pub fn new() -> CloudInstances {
        CloudInstances {
            count: None,
            next_href: None,
            prev_href: None,
            href: None,
            cloud_instance: None,
        }
    }
}


