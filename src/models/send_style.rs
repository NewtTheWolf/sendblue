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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
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
    #[serde(rename = "")]
    Default,
}
