//! Status Model
//!
//! This module provides the data model for message statuses used in the Sendblue API.

use serde::{Deserialize, Serialize};

/// Status of the message in the Sendblue API
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Queued,
    Failed,
    Sent,
    Delivered,
    Read,
}
