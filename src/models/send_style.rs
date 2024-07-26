//! Send Style Model
//!
//! This module provides the data model for send styles used in the Sendblue API.

use serde::{Deserialize, Serialize};

/// Style of the message delivery
///
/// # Examples
///
/// ```
/// use sendblue::models::SendStyle;
///
/// let style = SendStyle::Celebration;
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub enum SendStyle {
    Celebration,
    ShootingStar,
    Fireworks,
    Lasers,
    Love,
    Confetti,
    Balloons,
    Spotlight,
    Echo,
    Invisible,
    Gentle,
    Loud,
    Slam,
    Default,
}
