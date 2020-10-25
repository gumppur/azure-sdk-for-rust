#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[serde(rename = "accessKey")]
    pub access_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountInfo {
    pub name: String,
    pub properties: StorageAccountProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainerProperties {
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobContainer {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlobContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListBlobContainersResult {
    #[serde(skip_serializing)]
    pub value: Vec<BlobContainer>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasTokenInfo {
    #[serde(rename = "accessToken", skip_serializing)]
    pub access_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListSasTokensResult {
    #[serde(skip_serializing)]
    pub value: Vec<SasTokenInfo>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountInfoProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeStoreAccountInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountInfoProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountListStorageAccountsResult {
    #[serde(skip_serializing)]
    pub value: Vec<StorageAccountInfo>,
    #[serde(skip_serializing)]
    pub count: Option<i32>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountListDataLakeStoreResult {
    #[serde(skip_serializing)]
    pub value: Vec<DataLakeStoreAccountInfo>,
    #[serde(skip_serializing)]
    pub count: Option<i32>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<data_lake_analytics_account_properties::ProvisioningState>,
    #[serde(skip_serializing)]
    pub state: Option<data_lake_analytics_account_properties::State>,
    #[serde(rename = "defaultDataLakeStoreAccount", skip_serializing_if = "Option::is_none")]
    pub default_data_lake_store_account: Option<String>,
    #[serde(rename = "maxDegreeOfParallelism", skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[serde(rename = "maxJobCount", skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[serde(rename = "dataLakeStoreAccounts", skip_serializing_if = "Vec::is_empty")]
    pub data_lake_store_accounts: Vec<DataLakeStoreAccountInfo>,
    #[serde(rename = "storageAccounts", skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<StorageAccountInfo>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "lastModifiedTime", skip_serializing)]
    pub last_modified_time: Option<String>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
}
mod data_lake_analytics_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Failed,
        Creating,
        Running,
        Succeeded,
        Patching,
        Suspending,
        Resuming,
        Deleting,
        Deleted,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "suspended")]
        Suspended,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDataLakeStoreParameters {
    pub properties: DataLakeStoreAccountInfoProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountParameters {
    pub properties: StorageAccountProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeAnalyticsAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsAccountListResult {
    #[serde(skip_serializing)]
    pub value: Vec<DataLakeAnalyticsAccount>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(skip_serializing)]
    pub trace: Option<String>,
    #[serde(skip_serializing)]
    pub context: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetails>,
    #[serde(rename = "innerError", skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAsyncOperationResult {
    #[serde(skip_serializing)]
    pub status: Option<azure_async_operation_result::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
mod azure_async_operation_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
    }
}
