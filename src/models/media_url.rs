//! Media URL Model
//!
//! This module provides the data model for media URLs used in the Sendblue API.

use crate::Url;
use serde::{Deserialize, Serialize};
use url::Url as RawUrl;
use validator::ValidationError;

/// A URL for general media, can be any valid URL
///
/// # Examples
///
/// ```
/// use sendblue::models::MediaUrl;
/// use crate::sendblue::Url;
///
/// let media_url = MediaUrl::new("https://example.com/media.jpg").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaUrl(RawUrl);

impl Url for MediaUrl {
    fn new(url: &str) -> Result<Self, ValidationError> {
        let raw_url = RawUrl::parse(url).map_err(|_| ValidationError::new("invalid url format"))?;
        Ok(Self(raw_url))
    }

    fn from_raw_url(raw_url: RawUrl) -> Self {
        Self(raw_url)
    }

    fn raw_url(&self) -> &RawUrl {
        &self.0
    }
}

impl Serialize for MediaUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for MediaUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        MediaUrl::new(s).map_err(serde::de::Error::custom)
    }
}
