#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppProperties {
    #[serde(rename = "applicationId", skip_serializing)]
    pub application_id: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSkuInfo {
    pub name: app_sku_info::Name,
}
mod app_sku_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        F1,
        S1,
        #[serde(rename = "ST0")]
        St0,
        #[serde(rename = "ST1")]
        St1,
        #[serde(rename = "ST2")]
        St2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppProperties>,
    pub sku: AppSkuInfo,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<AppSkuInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<App>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInputs {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppAvailabilityInfo {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppTemplate {
    #[serde(rename = "manifestId", skip_serializing)]
    pub manifest_id: Option<String>,
    #[serde(rename = "manifestVersion", skip_serializing)]
    pub manifest_version: Option<String>,
    #[serde(rename = "appTemplateName", skip_serializing)]
    pub app_template_name: Option<String>,
    #[serde(skip_serializing)]
    pub title: Option<String>,
    #[serde(skip_serializing)]
    pub order: Option<f64>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppTemplatesResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing)]
    pub value: Vec<AppTemplate>,
}
