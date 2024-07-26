//! Evaluate Service Model
//!
//! This module provides the data models for evaluating if a number can send/receive iMessages,
//! including the request and response structures.

use phonenumber::PhoneNumber;
use serde::{Deserialize, Serialize};

/// Request parameters for evaluating if a number can send/receive iMessages
///
/// # Examples
///
/// ```
/// use sendblue::models::EvaluateServiceBuilder;
///
/// let request = EvaluateServiceBuilder::new()
///     .number(phonenumber::parse(None, "+19999999999").unwrap())
///     .build();
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateService {
    pub number: PhoneNumber,
}

/// Builder for creating an `EvaluateService` request
///
/// # Examples
///
/// ```
/// use sendblue::models::EvaluateServiceBuilder;
///
/// let request = EvaluateServiceBuilder::new()
///     .number(phonenumber::parse(None, "+19999999999").unwrap())
///     .build();
/// ```
pub struct EvaluateServiceBuilder {
    number: Option<PhoneNumber>,
}

impl EvaluateServiceBuilder {
    pub fn new() -> Self {
        Self { number: None }
    }

    pub fn number(mut self, number: PhoneNumber) -> Self {
        self.number = Some(number);
        self
    }

    pub fn build(self) -> EvaluateService {
        EvaluateService {
            number: self.number.expect("Number is required"),
        }
    }
}

/// Response from the Sendblue API for evaluating if a number can send/receive iMessages
///
/// # Examples
///
/// ```
/// use sendblue::models::EvaluateServiceResponse;
///
/// let response = EvaluateServiceResponse {
///     number: "+19999999999".into(),
///     service: "iMessage".into(),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluateServiceResponse {
    pub number: String,
    pub service: String,
}
