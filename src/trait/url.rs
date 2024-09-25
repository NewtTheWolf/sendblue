//! URL Trait
//!
//! This module provides a trait for handling URLs with default implementations for common operations.

use url::Url as RawUrl;

use crate::SendblueError;

/// A trait for handling URLs with default implementations for common operations.
///
/// # Methods
///
/// * `new` - Creates a new instance of the implementing type from a URL string.
/// * `as_str` - Returns the URL as a string slice.
/// * `url` - Returns a reference to the `RawUrl`.
pub trait Url: Sized {
    /// Creates a new instance of the implementing type from a URL string.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if the URL is invalid.
    fn new(url: &str) -> Result<Self, SendblueError> {
        let raw_url = RawUrl::parse(url)
            .map_err(|_| SendblueError::ValidationError("invalid url format".to_owned()))?;
        Ok(Self::from_raw_url(raw_url))
    }

    /// Returns the URL as a string slice.
    fn as_str(&self) -> &str {
        self.url().as_str()
    }

    /// Returns a reference to the `RawUrl`.
    fn url(&self) -> &RawUrl {
        self.raw_url()
    }

    /// Creates a new instance of the implementing type from a `RawUrl`.
    ///
    /// # Arguments
    ///
    /// * `raw_url` - A `RawUrl` instance.
    fn from_raw_url(raw_url: RawUrl) -> Self;

    /// Returns a reference to the `RawUrl`.
    ///
    /// This method must be implemented by the type.
    fn raw_url(&self) -> &RawUrl;
}
