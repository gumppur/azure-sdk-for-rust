#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyArtifactsRequest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmTemplateInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Object>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactDeploymentStatusProperties {
    #[serde(rename = "deploymentStatus", skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "artifactsApplied", skip_serializing_if = "Option::is_none")]
    pub artifacts_applied: Option<i32>,
    #[serde(rename = "totalArtifacts", skip_serializing_if = "Option::is_none")]
    pub total_artifacts: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactInstallProperties {
    #[serde(rename = "artifactId", skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ArtifactParameterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactParameterProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "filePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "targetOsType", skip_serializing_if = "Option::is_none")]
    pub target_os_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Object>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactSourceProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSourceProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "sourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<artifact_source_properties::SourceType>,
    #[serde(rename = "folderPath", skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[serde(rename = "branchRef", skip_serializing_if = "Option::is_none")]
    pub branch_ref: Option<String>,
    #[serde(rename = "securityToken", skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<artifact_source_properties::Status>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
mod artifact_source_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceType {
        VsoGit,
        GitHub,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CostProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostInsight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CostInsightProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostInsightProperties {
    #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "vmCosts", skip_serializing_if = "Vec::is_empty")]
    pub vm_costs: Vec<VmCostProperties>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostPerDayProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(rename = "costType", skip_serializing_if = "Option::is_none")]
    pub cost_type: Option<cost_per_day_properties::CostType>,
}
mod cost_per_day_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CostType {
        Unavailable,
        Reported,
        Projected,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostProperties {
    #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<CostPerDayProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomImageProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomImageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm: Option<CustomImagePropertiesFromVm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vhd: Option<CustomImagePropertiesCustom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<custom_image_properties::OsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
mod custom_image_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomImagePropertiesCustom {
    #[serde(rename = "imageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "sysPrep", skip_serializing_if = "Option::is_none")]
    pub sys_prep: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomImagePropertiesFromVm {
    #[serde(rename = "sourceVmId", skip_serializing_if = "Option::is_none")]
    pub source_vm_id: Option<String>,
    #[serde(rename = "sysPrep", skip_serializing_if = "Option::is_none")]
    pub sys_prep: Option<bool>,
    #[serde(rename = "windowsOsInfo", skip_serializing_if = "Option::is_none")]
    pub windows_os_info: Option<WindowsOsInfo>,
    #[serde(rename = "linuxOsInfo", skip_serializing_if = "Option::is_none")]
    pub linux_os_info: Option<LinuxOsInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluatePoliciesProperties {
    #[serde(rename = "factName", skip_serializing_if = "Option::is_none")]
    pub fact_name: Option<String>,
    #[serde(rename = "factData", skip_serializing_if = "Option::is_none")]
    pub fact_data: Option<String>,
    #[serde(rename = "valueOffset", skip_serializing_if = "Option::is_none")]
    pub value_offset: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluatePoliciesRequest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<EvaluatePoliciesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluatePoliciesResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<PolicySetResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Formula {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FormulaProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "formulaContent", skip_serializing_if = "Option::is_none")]
    pub formula_content: Option<LabVirtualMachine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm: Option<FormulaPropertiesFromVm>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaPropertiesFromVm {
    #[serde(rename = "labVmId", skip_serializing_if = "Option::is_none")]
    pub lab_vm_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "imageReference", skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<GalleryImageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateArmTemplateRequest {
    #[serde(rename = "virtualMachineName", skip_serializing_if = "Option::is_none")]
    pub virtual_machine_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateUploadUriParameter {
    #[serde(rename = "blobName", skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateUploadUriResponse {
    #[serde(rename = "uploadUri", skip_serializing_if = "Option::is_none")]
    pub upload_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lab {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabProperties {
    #[serde(rename = "defaultStorageAccount", skip_serializing_if = "Option::is_none")]
    pub default_storage_account: Option<String>,
    #[serde(rename = "artifactsStorageAccount", skip_serializing_if = "Option::is_none")]
    pub artifacts_storage_account: Option<String>,
    #[serde(rename = "storageAccounts", skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<String>,
    #[serde(rename = "vaultName", skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[serde(rename = "labStorageType", skip_serializing_if = "Option::is_none")]
    pub lab_storage_type: Option<lab_properties::LabStorageType>,
    #[serde(rename = "defaultVirtualNetworkId", skip_serializing_if = "Option::is_none")]
    pub default_virtual_network_id: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
mod lab_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LabStorageType {
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabVhd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabVirtualMachine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabVirtualMachineProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabVirtualMachineProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "ownerObjectId", skip_serializing_if = "Option::is_none")]
    pub owner_object_id: Option<String>,
    #[serde(rename = "createdByUserId", skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[serde(rename = "createdByUser", skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<String>,
    #[serde(rename = "computeId", skip_serializing_if = "Option::is_none")]
    pub compute_id: Option<String>,
    #[serde(rename = "customImageId", skip_serializing_if = "Option::is_none")]
    pub custom_image_id: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "sshKey", skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
    #[serde(rename = "isAuthenticationWithSshKey", skip_serializing_if = "Option::is_none")]
    pub is_authentication_with_ssh_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "labSubnetName", skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[serde(rename = "labVirtualNetworkId", skip_serializing_if = "Option::is_none")]
    pub lab_virtual_network_id: Option<String>,
    #[serde(rename = "disallowPublicIpAddress", skip_serializing_if = "Option::is_none")]
    pub disallow_public_ip_address: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
    #[serde(rename = "artifactDeploymentStatus", skip_serializing_if = "Option::is_none")]
    pub artifact_deployment_status: Option<ArtifactDeploymentStatusProperties>,
    #[serde(rename = "galleryImageReference", skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<GalleryImageReference>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxOsInfo {
    #[serde(rename = "linuxOsState", skip_serializing_if = "Option::is_none")]
    pub linux_os_state: Option<linux_os_info::LinuxOsState>,
}
mod linux_os_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LinuxOsState {
        NonDeprovisioned,
        DeprovisionRequested,
        DeprovisionApplied,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Object {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<policy_properties::Status>,
    #[serde(rename = "factName", skip_serializing_if = "Option::is_none")]
    pub fact_name: Option<policy_properties::FactName>,
    #[serde(rename = "factData", skip_serializing_if = "Option::is_none")]
    pub fact_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    #[serde(rename = "evaluatorType", skip_serializing_if = "Option::is_none")]
    pub evaluator_type: Option<policy_properties::EvaluatorType>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
mod policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FactName {
        UserOwnedLabVmCount,
        LabVmCount,
        LabVmSize,
        GalleryImage,
        UserOwnedLabVmCountInSubnet,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EvaluatorType {
        AllowedValuesPolicy,
        MaxValuePolicy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySetResult {
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "policyViolations", skip_serializing_if = "Vec::is_empty")]
    pub policy_violations: Vec<PolicyViolation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyViolation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationArtifact {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationArtifactSource {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArtifactSource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationCost {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cost>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationCostInsight {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CostInsight>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationCustomImage {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomImage>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationFormula {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Formula>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationGalleryImage {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GalleryImage>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationLab {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Lab>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationLabVhd {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabVhd>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationLabVirtualMachine {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabVirtualMachine>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationPolicy {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Policy>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationSchedule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWithContinuationVirtualNetwork {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<schedule_properties::Status>,
    #[serde(rename = "taskType", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<schedule_properties::TaskType>,
    #[serde(rename = "weeklyRecurrence", skip_serializing_if = "Option::is_none")]
    pub weekly_recurrence: Option<WeekDetails>,
    #[serde(rename = "dailyRecurrence", skip_serializing_if = "Option::is_none")]
    pub daily_recurrence: Option<DayDetails>,
    #[serde(rename = "hourlyRecurrence", skip_serializing_if = "Option::is_none")]
    pub hourly_recurrence: Option<HourDetails>,
    #[serde(rename = "timeZoneId", skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
mod schedule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TaskType {
        LabVmsShutdownTask,
        LabVmsStartupTask,
        LabBillingTask,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "labSubnetName", skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[serde(rename = "allowPublicIp", skip_serializing_if = "Option::is_none")]
    pub allow_public_ip: Option<subnet::AllowPublicIp>,
}
mod subnet {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllowPublicIp {
        Default,
        Deny,
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetOverride {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "labSubnetName", skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[serde(rename = "useInVmCreationPermission", skip_serializing_if = "Option::is_none")]
    pub use_in_vm_creation_permission: Option<subnet_override::UseInVmCreationPermission>,
    #[serde(rename = "usePublicIpAddressPermission", skip_serializing_if = "Option::is_none")]
    pub use_public_ip_address_permission: Option<subnet_override::UsePublicIpAddressPermission>,
}
mod subnet_override {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UseInVmCreationPermission {
        Default,
        Deny,
        Allow,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UsePublicIpAddressPermission {
        Default,
        Deny,
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionNotification {
    #[serde(rename = "registrationDate", skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_notification::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionNotificationProperties>,
}
mod subscription_notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        NotDefined,
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionNotificationProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetwork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkProperties {
    #[serde(rename = "allowedSubnets", skip_serializing_if = "Vec::is_empty")]
    pub allowed_subnets: Vec<Subnet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalProviderResourceId", skip_serializing_if = "Option::is_none")]
    pub external_provider_resource_id: Option<String>,
    #[serde(rename = "subnetOverrides", skip_serializing_if = "Vec::is_empty")]
    pub subnet_overrides: Vec<SubnetOverride>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmCostProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceGroupName", skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeekDetails {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weekdays: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsOsInfo {
    #[serde(rename = "windowsOsState", skip_serializing_if = "Option::is_none")]
    pub windows_os_state: Option<windows_os_info::WindowsOsState>,
}
mod windows_os_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WindowsOsState {
        NonSysprepped,
        SysprepRequested,
        SysprepApplied,
    }
}
