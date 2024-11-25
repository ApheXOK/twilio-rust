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
pub enum MessageFeedbackEnumOutcome {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "unconfirmed")]
    Unconfirmed,

}

impl std::fmt::Display for MessageFeedbackEnumOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Confirmed => write!(f, "confirmed"),
            Self::Unconfirmed => write!(f, "unconfirmed"),
        }
    }
}

impl Default for MessageFeedbackEnumOutcome {
    fn default() -> MessageFeedbackEnumOutcome {
        Self::Confirmed
    }
}

