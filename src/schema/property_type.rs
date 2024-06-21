use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{ Visitor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PropertyType {
    String="string",
    Number="number",
    Boolean="boolean",
    Array="array",
    Object="object",
}

impl Serialize for PropertyType {
    /// Serializes a `PropertyType` into an integer value.
    ///
    /// # Arguments
    ///
    /// * `serializer` - The serializer to use for converting the `PropertyType`.
    ///
    /// # Returns
    ///
    /// * `Result<S::Ok, S::Error>` - A result indicating success or failure.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let value = match self {
            PropertyType::String => 0,
            PropertyType::Number => 1,
            PropertyType::Boolean => 2,
            PropertyType::Array => 3,
            PropertyType::Object => 4,
            _ => 0,
        };
        serializer.serialize_i64(value)
    }
}

impl<'de> Deserialize<'de> for PropertyType {
    /// Deserializes an integer value into a `PropertyType`.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - The deserializer to use for converting the integer value.
    ///
    /// # Returns
    ///
    /// * `Result<Self, D::Error>` - A result indicating success or failure.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PropertyTypeVisitor)
    }
}

/// Visitor for deserializing a `PropertyType` from a string.
struct PropertyTypeVisitor;

impl<'de> Visitor<'de> for PropertyTypeVisitor {
    type Value = PropertyType;

    /// Describes what the visitor expects to receive.
    ///
    /// # Arguments
    ///
    /// * `formatter` - The formatter to use for displaying the expected value.
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - A result indicating success or failure.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 4 representing a PropertyType")
    }

    /// Visits a string value and attempts to convert it into a `PropertyType`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert.
    ///
    /// # Returns
    ///
    /// * `Result<PropertyType, E>` - A result indicating success or failure.
    fn visit_str<E>(self, value: &str) -> Result<PropertyType, E>
        where
            E: de::Error,
    {
        match value {
            "string" => Ok(PropertyType::String),
            "number" => Ok(PropertyType::Number),
            "boolean" => Ok(PropertyType::Boolean),
            "array" => Ok(PropertyType::Array),
            "object" => Ok(PropertyType::Object),
            _ => Err(E::invalid_value(de::Unexpected::Signed(value.parse().unwrap()), &self)),
        }
    }
}
