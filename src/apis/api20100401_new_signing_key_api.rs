/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_new_signing_key`]
#[derive(Clone, Debug)]
pub struct CreateNewSigningKeyParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Key resource.
    pub account_sid: String,
    /// A descriptive string that you create to describe the resource. It can be up to 64 characters long.
    pub friendly_name: Option<String>
}


/// struct for typed errors of method [`create_new_signing_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewSigningKeyError {
    UnknownValue(serde_json::Value),
}


/// Create a new Signing Key for the account making the request.
pub async fn create_new_signing_key(configuration: &configuration::Configuration, params: CreateNewSigningKeyParams) -> Result<models::ApiPeriodV2010PeriodAccountPeriodNewSigningKey, Error<CreateNewSigningKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let friendly_name = params.friendly_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/SigningKeys.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = friendly_name {
        local_var_form_params.insert("FriendlyName", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateNewSigningKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

