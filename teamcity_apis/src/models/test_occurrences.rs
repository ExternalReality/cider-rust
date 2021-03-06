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
pub struct TestOccurrences {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "nextHref", skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
    #[serde(rename = "prevHref", skip_serializing_if = "Option::is_none")]
    pub prev_href: Option<String>,
    #[serde(rename = "testOccurrence", skip_serializing_if = "Option::is_none")]
    pub test_occurrence: Option<Vec<crate::models::TestOccurrence>>,
    #[serde(rename = "testCounters", skip_serializing_if = "Option::is_none")]
    pub test_counters: Option<crate::models::TestCounters>,
    #[serde(rename = "passed", skip_serializing_if = "Option::is_none")]
    pub passed: Option<i32>,
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    #[serde(rename = "newFailed", skip_serializing_if = "Option::is_none")]
    pub new_failed: Option<i32>,
    #[serde(rename = "ignored", skip_serializing_if = "Option::is_none")]
    pub ignored: Option<i32>,
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<i32>,
}

impl TestOccurrences {
    pub fn new() -> TestOccurrences {
        TestOccurrences {
            count: None,
            href: None,
            next_href: None,
            prev_href: None,
            test_occurrence: None,
            test_counters: None,
            passed: None,
            failed: None,
            new_failed: None,
            ignored: None,
            muted: None,
        }
    }
}


