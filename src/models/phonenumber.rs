use phonenumber::{parse, PhoneNumber as RawPhoneNumber};
use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::ops::Deref;

/* #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PhoneNumber(pub String); */

/* // Delegate dereferencing to the inner type
impl Deref for PhoneNumber {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement Display for PhoneNumber to enable to_string method
impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
} */

/* // Implement JsonSchema for PhoneNumber
impl JsonSchema for PhoneNumber {
    fn schema_name() -> String {
        "PhoneNumber".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let schema_object = SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            format: Some("phone".to_string()),
            ..Default::default()
        };
        Schema::Object(schema_object)
    }
}
 */
/// Deserialize function for PhoneNumber
pub fn deserialize_phone_number<'de, D>(deserializer: D) -> Result<PhoneNumber, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    parse(None, s)
        .map(PhoneNumber)
        .map_err(serde::de::Error::custom)
}
