#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod servers {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get_details(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<AnalysisServicesServer, get_details::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get_details::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get_details::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_details::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServer = serde_json::from_slice(&body).context(get_details::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_details::ResponseBytesError)?;
                get_details::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get_details {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        server_parameters: &AnalysisServicesServer,
        subscription_id: &str,
    ) -> std::result::Result<create::Response, create::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(server_parameters);
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServer = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServer = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                create::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(AnalysisServicesServer),
            Created201(AnalysisServicesServer),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        server_update_parameters: &AnalysisServicesServerUpdateParameters,
        subscription_id: &str,
    ) -> std::result::Result<update::Response, update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(server_update_parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServer = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServer = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Accepted202(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(AnalysisServicesServer),
            Accepted202(AnalysisServicesServer),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                delete::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn suspend(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<suspend::Response, suspend::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}/suspend",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(suspend::BuildRequestError)?;
        let rsp = client.execute(req).await.context(suspend::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(suspend::Response::Ok200),
            StatusCode::ACCEPTED => Ok(suspend::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(suspend::ResponseBytesError)?;
                suspend::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod suspend {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn resume(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<resume::Response, resume::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}/resume",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(resume::BuildRequestError)?;
        let rsp = client.execute(req).await.context(resume::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(resume::Response::Ok200),
            StatusCode::ACCEPTED => Ok(resume::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(resume::ResponseBytesError)?;
                resume::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod resume {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<AnalysisServicesServers, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServers =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<AnalysisServicesServers, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AnalysisServices/servers",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: AnalysisServicesServers = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_skus_for_new(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<SkuEnumerationForNewResourceResult, list_skus_for_new::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AnalysisServices/skus",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_skus_for_new::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_skus_for_new::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_skus_for_new::ResponseBytesError)?;
                let rsp_value: SkuEnumerationForNewResourceResult =
                    serde_json::from_slice(&body).context(list_skus_for_new::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_skus_for_new::ResponseBytesError)?;
                list_skus_for_new::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_skus_for_new {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_skus_for_existing(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<SkuEnumerationForExistingResourceResult, list_skus_for_existing::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}/skus",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_skus_for_existing::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_skus_for_existing::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_skus_for_existing::ResponseBytesError)?;
                let rsp_value: SkuEnumerationForExistingResourceResult =
                    serde_json::from_slice(&body).context(list_skus_for_existing::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_skus_for_existing::ResponseBytesError)?;
                list_skus_for_existing::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_skus_for_existing {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_gateway_status(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<GatewayListStatusLive, list_gateway_status::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}/listGatewayStatus",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_gateway_status::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_gateway_status::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_gateway_status::ResponseBytesError)?;
                let rsp_value: GatewayListStatusLive =
                    serde_json::from_slice(&body).context(list_gateway_status::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_gateway_status::ResponseBytesError)?;
                let rsp_value: GatewayListStatusError =
                    serde_json::from_slice(&body).context(list_gateway_status::DeserializeError { body })?;
                list_gateway_status::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_gateway_status {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::GatewayListStatusError,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
    pub async fn dissociate_gateway(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<(), dissociate_gateway::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AnalysisServices/servers/{}/dissociateGateway",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(dissociate_gateway::BuildRequestError)?;
        let rsp = client.execute(req).await.context(dissociate_gateway::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(()),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(dissociate_gateway::ResponseBytesError)?;
                dissociate_gateway::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod dissociate_gateway {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn check_name_availability(
        operation_config: &crate::OperationConfig,
        location: &str,
        server_parameters: &CheckServerNameAvailabilityParameters,
        subscription_id: &str,
    ) -> std::result::Result<CheckServerNameAvailabilityResult, check_name_availability::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AnalysisServices/locations/{}/checkNameAvailability",
            &operation_config.base_path, subscription_id, location
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(server_parameters);
        let req = req_builder.build().context(check_name_availability::BuildRequestError)?;
        let rsp = client.execute(req).await.context(check_name_availability::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                let rsp_value: CheckServerNameAvailabilityResult =
                    serde_json::from_slice(&body).context(check_name_availability::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                check_name_availability::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod check_name_availability {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_operation_results(
        operation_config: &crate::OperationConfig,
        location: &str,
        operation_id: &str,
        subscription_id: &str,
    ) -> std::result::Result<list_operation_results::Response, list_operation_results::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AnalysisServices/locations/{}/operationresults/{}",
            &operation_config.base_path, subscription_id, location, operation_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_operation_results::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_operation_results::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(list_operation_results::Response::Ok200),
            StatusCode::ACCEPTED => Ok(list_operation_results::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_operation_results::ResponseBytesError)?;
                list_operation_results::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_operation_results {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_operation_statuses(
        operation_config: &crate::OperationConfig,
        location: &str,
        operation_id: &str,
        subscription_id: &str,
    ) -> std::result::Result<list_operation_statuses::Response, list_operation_statuses::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AnalysisServices/locations/{}/operationstatuses/{}",
            &operation_config.base_path, subscription_id, location, operation_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_operation_statuses::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_operation_statuses::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_operation_statuses::ResponseBytesError)?;
                let rsp_value: OperationStatus =
                    serde_json::from_slice(&body).context(list_operation_statuses::DeserializeError { body })?;
                Ok(list_operation_statuses::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_operation_statuses::ResponseBytesError)?;
                let rsp_value: OperationStatus =
                    serde_json::from_slice(&body).context(list_operation_statuses::DeserializeError { body })?;
                Ok(list_operation_statuses::Response::Accepted202(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_operation_statuses::ResponseBytesError)?;
                list_operation_statuses::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_operation_statuses {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(OperationStatus),
            Accepted202(OperationStatus),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
