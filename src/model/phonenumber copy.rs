use chrono::{DateTime, Utc};
use phonenumber::{parse, Mode, PhoneNumber as RawPhoneNumber};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, skip_serializing_none, NoneAsEmptyString};
use std::ops::Deref;
use validator::Validate;

#[cfg(feature = "schemars")]
use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
    JsonSchema,
};

// Newtype um PhoneNumber
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber(RawPhoneNumber);

impl PhoneNumber {
    pub fn new(phone_number: &str) -> Result<Self, phonenumber::ParseError> {
        parse(None, phone_number).map(PhoneNumber)
    }
}

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

impl From<RawPhoneNumber> for PhoneNumber {
    fn from(phone_number: RawPhoneNumber) -> Self {
        PhoneNumber(phone_number)
    }
}

// Deserialize a single PhoneNumber
pub fn deserialize_phone_number<'de, D>(deserializer: D) -> Result<PhoneNumber, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    parse(None, s)
        .map(PhoneNumber::from)
        .map_err(serde::de::Error::custom)
}

// Deserialize an Option<PhoneNumber>
pub fn deserialize_option_phone_number<'de, D>(
    deserializer: D,
) -> Result<Option<PhoneNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<&str> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => parse(None, s)
            .map(PhoneNumber::from)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

// Deserialize a Vec<PhoneNumber>
pub fn deserialize_vec_phone_number<'de, D>(deserializer: D) -> Result<Vec<PhoneNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<&str> = Deserialize::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| {
            parse(None, s)
                .map(PhoneNumber::from)
                .map_err(serde::de::Error::custom)
        })
        .collect()
}

// Deserialize an Option<Vec<PhoneNumber>>
pub fn deserialize_option_vec_phone_number<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<PhoneNumber>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Vec<&str>> = Option::deserialize(deserializer)?;
    match opt {
        Some(vec) => {
            let mut phone_numbers = Vec::new();
            for s in vec {
                match parse(None, s) {
                    Ok(phone_number) => phone_numbers.push(PhoneNumber::from(phone_number)),
                    Err(err) => return Err(serde::de::Error::custom(err)),
                }
            }
            Ok(Some(phone_numbers))
        }
        None => Ok(None),
    }
}

// Serialize a PhoneNumber to a string in E164 format
pub fn serialize_phone_number<S>(number: &PhoneNumber, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let e164 = number.format().mode(Mode::E164).to_string();
    serializer.serialize_str(&e164)
}
