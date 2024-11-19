/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayloadPeriodRecordingAddOnResultPayloadData {
    /// The URL to redirect to to get the data returned by the AddOn that was previously stored.
    #[serde(rename = "redirect_to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayloadPeriodRecordingAddOnResultPayloadData {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayloadPeriodRecordingAddOnResultPayloadData {
        ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayloadPeriodRecordingAddOnResultPayloadData {
            redirect_to: None,
        }
    }
}

