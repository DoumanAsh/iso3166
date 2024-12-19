use crate::Country;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use core::fmt;

impl Serialize for Country {
    #[inline(always)]
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.0.alpha3)
    }
}

impl<'a> Deserialize<'a> for Country {
    fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
        des.deserialize_str(CountryVisitor)
    }
}

struct CountryVisitor;

impl<'a> Visitor<'a> for CountryVisitor {
    type Value = Country;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expected 2 or 3 digit ISO-3166 country code")
    }

    #[inline]
    fn visit_str<E: Error>(self, input: &str) -> Result<Self::Value, E> {
        if input.len() == 2 {
            match Country::from_alpha2_ignore_case(input) {
                Some(country) => Ok(country),
                None => Err(Error::invalid_value(serde::de::Unexpected::Str(input), &self))
            }
        } else if input.len() == 3 {
            match Country::from_alpha3_ignore_case(input) {
                Some(country) => Ok(country),
                None => Err(Error::invalid_value(serde::de::Unexpected::Str(input), &self))
            }
        } else {
            Err(Error::invalid_value(serde::de::Unexpected::Str(input), &self))
        }
    }
}
