use std::ops::Deref;
use validators::prelude::*;
use validators_prelude::phonenumber::PhoneNumber as RawPhoneNumber;

#[cfg(feature = "schemars")]
use schemars::{
    JsonSchema,
    {
        gen::SchemaGenerator,
        schema::{Schema, SchemaObject},
    },
};

#[derive(Validator, Debug, Clone, PartialEq, Eq)]
#[validator(phone)]
pub struct PhoneNumber(pub RawPhoneNumber);

impl Deref for PhoneNumber {
    type Target = RawPhoneNumber;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement JsonSchema for PhoneNumber
#[cfg(feature = "schemars")]
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
