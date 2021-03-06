#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod vm_insights {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get_onboarding_status(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
    ) -> std::result::Result<VmInsightsOnboardingStatus, get_onboarding_status::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/{}/providers/Microsoft.Insights/vmInsightsOnboardingStatuses/default",
            &operation_config.base_path, resource_uri
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get_onboarding_status::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get_onboarding_status::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get_onboarding_status::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_onboarding_status::ResponseBytesError)?;
                let rsp_value: VmInsightsOnboardingStatus =
                    serde_json::from_slice(&body).context(get_onboarding_status::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_onboarding_status::ResponseBytesError)?;
                let rsp_value: ResponseWithError =
                    serde_json::from_slice(&body).context(get_onboarding_status::DeserializeError { body })?;
                get_onboarding_status::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get_onboarding_status {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ResponseWithError,
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
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}
