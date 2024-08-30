//! r#trait for the Sendblue API
//!
//! This module provides r#trait used by various models in the Sendblue API.

pub mod sendable_message;
pub mod url;

pub use sendable_message::SendableMessage;
pub use url::Url;
