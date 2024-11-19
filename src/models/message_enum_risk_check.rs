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
pub enum MessageEnumRiskCheck {
    #[serde(rename = "enable")]
    Enable,
    #[serde(rename = "disable")]
    Disable,

}

impl std::fmt::Display for MessageEnumRiskCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Enable => write!(f, "enable"),
            Self::Disable => write!(f, "disable"),
        }
    }
}

impl Default for MessageEnumRiskCheck {
    fn default() -> MessageEnumRiskCheck {
        Self::Enable
    }
}

