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
pub struct RelatedEntities {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Vec<crate::models::RelatedEntity>>,
}

impl RelatedEntities {
    pub fn new() -> RelatedEntities {
        RelatedEntities {
            count: None,
            entity: None,
        }
    }
}


