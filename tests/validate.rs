use iso3166::{Country, LIST};

use core::mem;
use std::collections::HashSet;

#[test]
fn should_be_valid() {
    assert_eq!(mem::size_of::<Country>(), mem::size_of::<usize>());
    assert_eq!(mem::size_of::<Option<Country>>(), mem::size_of::<usize>());
    assert_eq!(mem::size_of::<Result<Country, ()>>(), mem::size_of::<usize>());

    let mut countries = HashSet::new();
    let mut ids = HashSet::new();
    let mut name = HashSet::new();
    let mut alpha3 = HashSet::new();
    let mut alpha2 = HashSet::new();
    for country in LIST {
        assert_eq!(country.alpha2.len(), 2);
        assert_eq!(country.alpha3.len(), 3);

        assert!(countries.insert(country));
        assert!(ids.insert(country.id));
        assert!(name.insert(country.name));
        assert!(alpha3.insert(country.alpha3));
        assert!(alpha2.insert(country.alpha2));

        assert_eq!(country, Country::from_id(country.id).unwrap());

        assert_eq!(country, Country::from_alpha2(country.alpha2).unwrap());
        assert!(Country::from_alpha2(&country.alpha2.to_lowercase()).is_none());
        assert_eq!(country, Country::from_alpha2_ignore_case(country.alpha2).unwrap());
        assert_eq!(country, Country::from_alpha2_ignore_case(&country.alpha2.to_lowercase()).unwrap());

        assert_eq!(country, Country::from_alpha3(country.alpha3).unwrap());
        assert!(Country::from_alpha3(&country.alpha3.to_lowercase()).is_none());
        assert_eq!(country, Country::from_alpha3_ignore_case(country.alpha3).unwrap());
        assert_eq!(country, Country::from_alpha3_ignore_case(&country.alpha3.to_lowercase()).unwrap());
    }
}

#[cfg(feature = "serde")]
#[test]
fn should_validate_serde() {
    for country in LIST.iter() {
        let string = serde_json::to_string(country).expect("to serialize");
        let result = serde_json::from_str::<Country>(&string).expect("to deserialize");
        assert_eq!(country, &result);
        let result = serde_json::from_str::<Country>(&format!("\"{}\"", country.data().alpha3)).expect("to deserialize");
        assert_eq!(country, &result);
        let result = serde_json::from_str::<Country>(&format!("\"{}\"", country.data().alpha2)).expect("to deserialize");
        assert_eq!(country, &result);
    }
}
