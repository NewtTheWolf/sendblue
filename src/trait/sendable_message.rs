use serde::Serialize;

/// Trait for messages that can be sent
pub trait SendableMessage: Serialize {
    fn endpoint() -> &'static str;
    type ResponseType: for<'de> serde::Deserialize<'de>;
}
