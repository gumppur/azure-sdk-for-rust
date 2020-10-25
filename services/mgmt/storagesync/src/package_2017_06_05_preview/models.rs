#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<StorageSyncErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_state::State>,
    #[serde(skip_serializing)]
    pub istransitioning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionStateProperties>,
}
mod subscription_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServer {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesMoveInfo {
    #[serde(rename = "targetResourceGroup", skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostRestoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "failedFileList", skip_serializing_if = "Option::is_none")]
    pub failed_file_list: Option<String>,
    #[serde(rename = "restoreFileSpec", skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreRestoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "backupMetadataPropertyBag", skip_serializing_if = "Option::is_none")]
    pub backup_metadata_property_bag: Option<String>,
    #[serde(rename = "restoreFileSpec", skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
    #[serde(rename = "pauseWaitForSyncDrainTimePeriodInSeconds", skip_serializing_if = "Option::is_none")]
    pub pause_wait_for_sync_drain_time_period_in_seconds: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupRequest {
    #[serde(rename = "azureFileShare", skip_serializing_if = "Option::is_none")]
    pub azure_file_share: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostBackupResponse {
    #[serde(rename = "backupMetadata", skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<PostBackupResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreFileSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing)]
    pub isdir: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageSyncService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroupArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpointArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServerArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegisteredServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowArray {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionStateProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostBackupResponseProperties {
    #[serde(rename = "cloudEndpointName", skip_serializing)]
    pub cloud_endpoint_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceProperties {
    #[serde(rename = "storageSyncServiceStatus", skip_serializing)]
    pub storage_sync_service_status: Option<i64>,
    #[serde(rename = "storageSyncServiceUid", skip_serializing)]
    pub storage_sync_service_uid: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowProperties {
    #[serde(rename = "lastStepName", skip_serializing_if = "Option::is_none")]
    pub last_step_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    #[serde(rename = "lastOperationId", skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncGroupProperties {
    #[serde(rename = "uniqueId", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "syncGroupStatus", skip_serializing)]
    pub sync_group_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredServerProperties {
    #[serde(rename = "serverCertificate", skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "serverManagementtErrorCode", skip_serializing_if = "Option::is_none")]
    pub server_managementt_error_code: Option<i64>,
    #[serde(rename = "lastHeartBeat", skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "serverRole", skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "storageSyncServiceUid", skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
    #[serde(rename = "lastWorkflowId", skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudEndpointProperties {
    #[serde(rename = "storageAccountKey", skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
    #[serde(rename = "storageAccountResourceId", skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "storageAccountShareName", skip_serializing_if = "Option::is_none")]
    pub storage_account_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[serde(rename = "partnershipId", skip_serializing_if = "Option::is_none")]
    pub partnership_id: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "backupEnabled", skip_serializing)]
    pub backup_enabled: Option<bool>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerEndpointProperties {
    #[serde(rename = "serverLocalPath", skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "lastSyncSuccess", skip_serializing_if = "Option::is_none")]
    pub last_sync_success: Option<String>,
    #[serde(rename = "syncErrorState", skip_serializing_if = "Option::is_none")]
    pub sync_error_state: Option<String>,
    #[serde(rename = "syncErrorStateTimestamp", skip_serializing_if = "Option::is_none")]
    pub sync_error_state_timestamp: Option<String>,
    #[serde(rename = "syncErrorDirection", skip_serializing_if = "Option::is_none")]
    pub sync_error_direction: Option<ProgressType>,
    #[serde(rename = "itemUploadErrorCount", skip_serializing_if = "Option::is_none")]
    pub item_upload_error_count: Option<i64>,
    #[serde(rename = "itemDownloadErrorCount", skip_serializing_if = "Option::is_none")]
    pub item_download_error_count: Option<i64>,
    #[serde(rename = "syncErrorContext", skip_serializing_if = "Option::is_none")]
    pub sync_error_context: Option<String>,
    #[serde(rename = "currentProgressType", skip_serializing_if = "Option::is_none")]
    pub current_progress_type: Option<ProgressType>,
    #[serde(rename = "itemProgressCount", skip_serializing_if = "Option::is_none")]
    pub item_progress_count: Option<i64>,
    #[serde(rename = "itemTotalCount", skip_serializing_if = "Option::is_none")]
    pub item_total_count: Option<i64>,
    #[serde(rename = "byteProgress", skip_serializing_if = "Option::is_none")]
    pub byte_progress: Option<i64>,
    #[serde(rename = "totalProgress", skip_serializing_if = "Option::is_none")]
    pub total_progress: Option<i64>,
    #[serde(rename = "byteTotal", skip_serializing_if = "Option::is_none")]
    pub byte_total: Option<i64>,
    #[serde(rename = "serverResourceId", skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicalPath {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceId {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsObject {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "failed")]
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationDirection {
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "cancel")]
    Cancel,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgressType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "recall")]
    Recall,
}
