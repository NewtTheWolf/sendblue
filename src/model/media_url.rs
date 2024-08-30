//! Media URL Model
//!
//! This module provides the data model for media URLs used in the Sendblue API.

use crate::r#trait::Url;
use schemars::{
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref, str::FromStr};
use url::Url as RawUrl;
use validator::ValidationError;

/// A URL for general media, can be any valid URL
///
/// # Examples
///
/// ```
/// use sendblue::models::MediaUrl;
/// use sendblue::r#trait::Url;
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

impl fmt::Display for MediaUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MediaUrl {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_url = RawUrl::parse(s).map_err(|_| ValidationError::new("invalid url format"))?;
        Ok(Self(raw_url))
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

impl Deref for MediaUrl {
    type Target = RawUrl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonSchema for MediaUrl {
    fn schema_name() -> String {
        "MediaUrl".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("uri".to_string()),
            ..Default::default()
        }
        .into()
    }
}
