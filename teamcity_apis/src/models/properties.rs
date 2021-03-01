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
pub struct Properties {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<crate::models::Property>>,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            count: None,
            href: None,
            property: None,
        }
    }
}


