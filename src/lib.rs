//! ISO-3166 country [data](https://www.datahub.io/core/country-codes) from 2023/10/27
//!
//! ## Features:
//!
//! - `serde` - Enables serialization impl with 3 digit codes

#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]
#![cfg_attr(rustfmt, rustfmt_skip)]

mod data;
pub mod countries;
pub use countries::LIST;
#[cfg(feature = "serde")]
mod serde;

use core::{cmp, hash, ops, fmt};

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
///Region
pub enum Region {
    ///Europe
    Europe,
    ///Asia
    Asia,
    ///North America
    ///
    ///Includes central America and Caribbean islands
    NorthAmerica,
    ///South America
    SouthAmerica,
    ///Africa
    Africa,
    ///Oceania
    Oceania,
}

impl Region {
    #[inline(always)]
    ///Gets region name
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Europe => "Europe",
            Self::Asia => "Asia",
            Self::NorthAmerica => "NorthAmerica",
            Self::SouthAmerica => "SouthAmerica",
            Self::Africa => "Africa",
            Self::Oceania => "Oceania",
        }
    }
}

impl fmt::Debug for Region {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

impl fmt::Display for Region {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

///Country data
pub struct Data {
    ///Unique numeric id assigned to the country
    ///
    ///Comparing to alpha2/alpha3, this ID can never be re-assigned to new country.
    ///Even if country has inherited alpha2/alpha3 codes
    pub id: u16,
    ///2 digit country code
    pub alpha2: &'static str,
    ///3 digit country code
    pub alpha3: &'static str,
    ///Country name
    pub name: &'static str,
    ///Region name
    pub region: Region,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
///Country
pub struct Country(&'static Data);

impl Country {
    #[inline(always)]
    ///Accesses country data
    ///
    ///`Country` implements `Deref` for it already, so you do not necessary need this method
    pub const fn data(&self) -> &'static Data {
        self.0
    }

    #[inline(always)]
    ///Converts from country id
    pub fn from_id(id: u16) -> Option<Self> {
        countries::from_id(id)
    }

    #[inline(always)]
    ///Converts from 2 digit country code
    pub fn from_alpha2(code: &str) -> Option<Self> {
        if code.len() == 2 {
            let code = code.as_bytes();
            return countries::from_alpha2([code[0], code[1]]);
        }
        None
    }

    #[inline(always)]
    ///Converts from 2 digit country code
    pub fn from_alpha2_ignore_case(code: &str) -> Option<Self> {
        if code.len() == 2 {
            let code = code.as_bytes();
            return countries::from_alpha2([code[0].to_ascii_uppercase(), code[1].to_ascii_uppercase()]);
        }
        None
    }

    #[inline(always)]
    ///Converts from 3 digit country code
    pub fn from_alpha3(code: &str) -> Option<Self> {
        if code.len() == 3 {
            let code = code.as_bytes();
            return countries::from_alpha3([code[0], code[1], code[2]]);
        }
        None
    }

    #[inline(always)]
    ///Converts from 3 digit country code
    pub fn from_alpha3_ignore_case(code: &str) -> Option<Self> {
        if code.len() == 3 {
            let code = code.as_bytes();
            return countries::from_alpha3([code[0].to_ascii_uppercase(), code[1].to_ascii_uppercase(), code[2].to_ascii_uppercase()]);
        }
        None
    }
}

impl PartialEq for Country {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

impl PartialEq<Country> for &Country {
    #[inline(always)]
    fn eq(&self, other: &Country) -> bool {
        self.0.id == other.0.id
    }
}

impl PartialEq<&Country> for Country {
    #[inline(always)]
    fn eq(&self, other: &&Self) -> bool {
        self.0.id == other.0.id
    }
}

impl Eq for Country {}

impl ops::Deref for Country {
    type Target = Data;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl hash::Hash for Country {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.id.hash(state)
    }
}

impl fmt::Debug for Country {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(self.name)
           .field("id", &self.id)
           .field("alpha2", &self.alpha2)
           .field("alpha3", &self.alpha3)
           .field("region", &self.region)
           .finish()
    }
}

impl cmp::PartialOrd for Country {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        cmp::PartialOrd::partial_cmp(&self.0.id, &other.0.id)
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        cmp::PartialOrd::lt(&self.0.id, &other.0.id)
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        cmp::PartialOrd::le(&self.0.id, &other.0.id)
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        cmp::PartialOrd::gt(&self.0.id, &other.0.id)
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        cmp::PartialOrd::ge(&self.0.id, &other.0.id)
    }
}

impl cmp::Ord for Country {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        cmp::Ord::cmp(&self.0.id, &other.0.id)
    }
}
