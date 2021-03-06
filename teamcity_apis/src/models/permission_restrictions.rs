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
pub struct PermissionRestrictions {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "permissionRestriction", skip_serializing_if = "Option::is_none")]
    pub permission_restriction: Option<Vec<crate::models::PermissionRestriction>>,
}

impl PermissionRestrictions {
    pub fn new() -> PermissionRestrictions {
        PermissionRestrictions {
            count: None,
            permission_restriction: None,
        }
    }
}


