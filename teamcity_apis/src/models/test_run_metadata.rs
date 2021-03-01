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
pub struct TestRunMetadata {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "typedValues", skip_serializing_if = "Option::is_none")]
    pub typed_values: Option<Vec<crate::models::TypedValue>>,
}

impl TestRunMetadata {
    pub fn new() -> TestRunMetadata {
        TestRunMetadata {
            count: None,
            typed_values: None,
        }
    }
}


