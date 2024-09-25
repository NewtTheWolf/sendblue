//! Callback URL Model
//!
//! This module provides the data model for callback URLs used in the Sendblue API.

use crate::{r#trait::Url, SendblueError};
use serde::{Deserialize, Serialize};
use url::Url as RawUrl;

/// A URL for status callback, must be a valid URL
///
/// # Examples
///
/// ```
/// use sendblue::models::CallbackUrl;
/// use sendblue::r#trait::Url;
///
/// let callback_url = CallbackUrl::new("https://example.com/callback").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallbackUrl(RawUrl);

impl Url for CallbackUrl {
    fn new(url: &str) -> Result<Self, SendblueError> {
        let url = RawUrl::parse(url)
            .map_err(|_| SendblueError::ValidationError("invalid url format".to_owned()))?;
        Ok(Self(url))
    }

    fn from_raw_url(raw_url: RawUrl) -> Self {
        Self(raw_url)
    }

    fn raw_url(&self) -> &RawUrl {
        &self.0
    }
}

impl Serialize for CallbackUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for CallbackUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        CallbackUrl::new(s).map_err(serde::de::Error::custom)
    }
}
