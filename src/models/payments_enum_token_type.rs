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
pub enum PaymentsEnumTokenType {
    #[serde(rename = "one-time")]
    OneTime,
    #[serde(rename = "reusable")]
    Reusable,
    #[serde(rename = "payment-method")]
    PaymentMethod,

}

impl std::fmt::Display for PaymentsEnumTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OneTime => write!(f, "one-time"),
            Self::Reusable => write!(f, "reusable"),
            Self::PaymentMethod => write!(f, "payment-method"),
        }
    }
}

impl Default for PaymentsEnumTokenType {
    fn default() -> PaymentsEnumTokenType {
        Self::OneTime
    }
}

