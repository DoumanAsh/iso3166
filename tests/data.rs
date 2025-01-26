use std::fs;
use std::io::Write;
use std::collections::{BTreeMap, HashSet};

use iso3166::LIST;

#[test]
fn should_re_generate_data() {
    const FILE: &str = "re_generated.csv";

    let out = fs::File::create(FILE).expect("create re_generated.csv");
    let mut csv = csv::WriterBuilder::new().has_headers(false).from_writer(out);

    let header = ("id", "alpha2", "alpha3", "name", "region");
    csv.serialize(header).expect("write csv");
    for country in LIST {
        let data = country.data();
        let fields = (data.id, data.alpha2, data.alpha3, data.name, data.region.as_str());
        csv.serialize(fields).expect("write csv");
    }
}

#[test]
fn should_re_generate_source_code() {
    const TAB: &str = "    ";
    const OUT_DATA: &str = "src/data.rs";
    const OUT_COUNTRIES: &str = "src/countries.rs";
    const FILE: &str = "data/countries.csv";

    let input = fs::File::open(FILE).expect("open data/countries.csv");
    let mut csv = csv::ReaderBuilder::new().has_headers(true).from_reader(input);

    struct Country {
        id: u64,
        alpha2: String,
        alpha3: String,
        name: String,
        region: String,
    }
    let mut countries = Vec::new();
    let mut buffer = csv::StringRecord::new();
    while csv.read_record(&mut buffer).expect("read record") {
        let id = buffer.get(0).expect("to have id field").parse().expect("valid u16 id");
        let alpha2 = buffer.get(1).expect("to have alpha2 field").to_owned();
        let alpha3 = buffer.get(2).expect("to have alpha3 field").to_owned();
        let name = buffer.get(3).expect("to have name field").to_owned();
        let region = buffer.get(4).expect("to have region field").to_owned();

        countries.push(Country {
            id,
            alpha2,
            alpha3,
            name,
            region
        })
    }

    assert_eq!(countries.len(), LIST.len());

    let mut numeric_ids = HashSet::new();
    let mut alpha2_map = BTreeMap::new();
    let mut alpha3_map = BTreeMap::new();
    for country in countries.iter() {
        let Country { id, alpha2, alpha3, .. } = country;

        assert!(numeric_ids.insert(id), "Repeat numeric id='{}' detected", id);
        if !alpha2.is_empty() {
            let mut chars = alpha2.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();
            assert!(chars.next().is_none(), "alpha2 should only have 2 chars");
            let inner_map = alpha2_map.entry(first).or_insert_with(BTreeMap::new);
            inner_map.insert(second, alpha3);
        }

        if !alpha3.is_empty() {
            let mut chars = alpha3.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();
            let third = chars.next().unwrap();
            assert!(chars.next().is_none(), "alpha3 should only have 3 chars");
            let inner_map = alpha3_map.entry(first).or_insert_with(BTreeMap::new);
            let inner_map2 = inner_map.entry(second).or_insert_with(BTreeMap::new);
            inner_map2.insert(third, alpha3);
        }
    }


    let mut out = fs::File::create(OUT_DATA).expect("open src");
    macro_rules! write_all {
        ($data:expr) => {
            out.write_all($data).expect("write_all src")

        };
    }
    macro_rules! write_fmt {
        ($($tokens:tt)*) => {
            out.write_fmt(format_args!($($tokens)*)).expect("write_fmt src")

        };
    }

    write_all!(b"use crate::{Region, Data};\n\n");
    for country in countries.iter() {
        let Country { id, alpha2, alpha3, name, region } = country;
        write_fmt!("pub const {alpha3}_ID: u16 = {id};\n");
        write_fmt!("pub const {alpha3}_ALPHA2: &str = \"{alpha2}\";\n");
        write_fmt!("pub const {alpha3}_ALPHA3: &str = \"{alpha3}\";\n");
        write_fmt!("pub const {alpha3}: Data = Data {{\n");
        write_fmt!("    id: {alpha3}_ID,\n");
        write_fmt!("    alpha2: {alpha3}_ALPHA2,\n");
        write_fmt!("    alpha3: {alpha3}_ALPHA3,\n");
        write_fmt!("    name: \"{name}\",\n");
        write_fmt!("    region: Region::{region},\n");
        write_all!(b"};\n");
    }
    out.flush().expect("to finish src/data.rs");

    out = fs::File::create(OUT_COUNTRIES).expect("open src/countries.rs");
    write_all!(b"//!Country enumeration\n\n");
    write_all!(b"use crate::{Country, data};\n\n");
    for country in countries.iter() {
        let Country { alpha3, name, .. } = country;
        write_fmt!("///{name}\n");
        write_fmt!("pub const {alpha3}: Country = Country(&data::{alpha3});\n");
    }

    write_all!(b"///List of countries\n");
    write_fmt!("pub const LIST: [&'static Country; {}] = [\n", countries.len());
    for country in countries.iter() {
        let Country { alpha3, .. } = country;
        write_fmt!("    &{alpha3},\n");
    }
    write_all!(b"];\n");

    //from_id()
    write_all!(b"\n///Look up country by its numeric ID\n");
    write_all!(b"pub const fn from_id(id: u16) -> Option<&'static Country> {\n");

    write_fmt!("{TAB}match id {{\n");
    for country in countries.iter() {
        let Country { alpha3, .. } = country;
        write_fmt!("{TAB}{TAB}data::{alpha3}_ID => Some(&{alpha3}),\n");
    }
    write_fmt!("{TAB}{TAB}_ => None,\n");
    write_fmt!("{TAB}}}\n");

    write_all!(b"}\n");

    //from_alpha2()
    write_all!(b"\n///Look up country by alpha2 code\n");
    write_all!(b"pub const fn from_alpha2(alpha2: [u8; 2]) -> Option<Country> {\n");

    write_fmt!("{TAB}match alpha2[0] {{\n");
    for (first, inner_map) in alpha2_map.iter() {
        write_fmt!("{TAB}{TAB}b'{first}' => match alpha2[1] {{\n");
        for (second, alpha3) in inner_map.iter() {
            write_fmt!("{TAB}{TAB}{TAB}b'{second}' => Some({alpha3}),\n");
        }
        write_fmt!("{TAB}{TAB}{TAB}_ => None,\n");
        write_fmt!("{TAB}{TAB}}},\n");
    }
    write_fmt!("{TAB}{TAB}_ => None,\n");
    write_fmt!("{TAB}}}\n");

    write_all!(b"}\n");

    //from_alpha3()
    write_all!(b"\n///Look up country by alpha3 code\n");
    write_all!(b"pub const fn from_alpha3(alpha3: [u8; 3]) -> Option<Country> {\n");

    write_fmt!("{TAB}match alpha3[0] {{\n");
    for (first, inner_map) in alpha3_map.iter() {
        write_fmt!("{TAB}{TAB}b'{first}' => match alpha3[1] {{\n");
        for (second, inner_map2) in inner_map.iter() {
            write_fmt!("{TAB}{TAB}{TAB}b'{second}' => match alpha3[2] {{\n");
            for (third, alpha3) in inner_map2.iter() {
                write_fmt!("{TAB}{TAB}{TAB}{TAB}b'{third}' => Some({alpha3}),\n");
            }
            write_fmt!("{TAB}{TAB}{TAB}{TAB}_ => None,\n");
            write_fmt!("{TAB}{TAB}{TAB}}},\n");
        }
        write_fmt!("{TAB}{TAB}{TAB}_ => None,\n");
        write_fmt!("{TAB}{TAB}}},\n");
    }
    write_fmt!("{TAB}{TAB}_ => None,\n");
    write_fmt!("{TAB}}}\n");

    write_all!(b"}\n");

}
