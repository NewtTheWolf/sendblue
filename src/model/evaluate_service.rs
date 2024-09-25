//! Evaluate Service Model
//!
//! This module provides the data models for evaluating if a number can send/receive iMessages,
//! including the request and response structures.

use super::PhoneNumber;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Request parameters for evaluating if a number can send/receive iMessages.
///
/// # Examples
///
/// ```
/// use sendblue::models::EvaluateService;
///
/// let request = EvaluateService::new(PhoneNumber::new("+1234567890").unwrap());
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateService {
    pub number: PhoneNumber,
}

impl EvaluateService {
    pub fn new(number: PhoneNumber) -> Self {
        Self { number }
    }
}

/// Enum for the type of service that can be evaluated
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum EvaluateServiceType {
    IMessage,
    SMS,
}

/// Response from the Sendblue API for evaluating if a number can send/receive iMessages
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EvaluateServiceResponse {
    pub number: PhoneNumber,
    pub service: EvaluateServiceType,
}
