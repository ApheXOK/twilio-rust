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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomingPhoneNumberEnumVoiceReceiveMode {
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "fax")]
    Fax,

}

impl std::fmt::Display for IncomingPhoneNumberEnumVoiceReceiveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Voice => write!(f, "voice"),
            Self::Fax => write!(f, "fax"),
        }
    }
}

impl Default for IncomingPhoneNumberEnumVoiceReceiveMode {
    fn default() -> IncomingPhoneNumberEnumVoiceReceiveMode {
        Self::Voice
    }
}

