// lib.rs

use std::fs;  // for reading directories.
use std::path::{Path, PathBuf};
use json::{JsonValue, object, Error as JSONError};

// Third Party crates
use chrono::prelude::*;
use regex::Regex;
#[allow(unused_imports)]
use serde::ser::{Serializer, SerializeStruct};
#[allow(unused_imports)]
// use serde_json::Result as ResultSerde;

pub mod stamp;

pub fn get_package_version() -> &'static str {
    // Completed.
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    VERSION
}

// --------

pub fn print_path_parts(path: &Path) -> () {
    println!("is_dir: {}", path.is_dir());
    println!("file_name {:?}", path.file_name());

    let has_parent: bool;
    match path.parent() {
        Some(x) => {
            if x.to_str().is_none() { has_parent = false; }
            else { has_parent = x.to_str().unwrap() != "" }
        },
        None => {
            has_parent = false;
        }            
    }
    println!("has_parent {:?}", has_parent);
    if has_parent {
        println!("parent: {:?}", path.parent().unwrap());
    }
    println!("file_stem {:?}", path.file_stem());
}

// Public Functions

// Converting DateTime to ISO 8601 Strings.

// Same as a Path, but must contain an ISO 8601 datetime in the stem
#[allow(dead_code)]
struct DatedPath {
    path: Path
}

// Same as PathBuf, but must contain an ISO 8601 datetime in the stem.
struct DatedPathBuf {
    pathbuf: PathBuf
}

impl DatedPathBuf {

    fn new_from_pathbuf(path_buffer: PathBuf) -> Result<DatedPathBuf, &'static str> {
        if path_buffer.to_str().is_none() {
            return Err("Path cannot be represented by a UTF8 String.");
        }

        if ! self::is_path_file8601(path_buffer.as_path()) {
            return Err("Path does not contain an ISO 8601 datetime.");
        }

        Ok(DatedPathBuf {
            pathbuf: path_buffer
        })
    }

    #[allow(dead_code)]
    fn json_metadata(&self) -> Result<JsonValue, JSONError> {
        let mut data: JsonValue = json::JsonValue::new_array();

        let inner = object!{
            name: self.pathbuf.to_str().unwrap(),
            age: 30,
            is_programmer: true
        };
        data.push(inner)?;
        Ok(data)
    }
}



pub fn datetime_from_string(dt_as_string: &str) -> Option<DateTime<FixedOffset>> {
    const FMT: &str = "%Y-%m-%dT%H%M%S%z";
    match DateTime::parse_from_str(dt_as_string, FMT) {
        #[allow(unused_variables)]
        Err(e) => {
            return None::<DateTime<FixedOffset>>;
        }
        Ok(f) => {
            return Some(f);
        }
    };
 }


pub fn build_metadata_from_path(parm_path: &Path, must_exist: bool) -> Result<String, String> {
    // Given a path (file or directory) build metadata.
    if must_exist && !parm_path.exists() {
        let ret = format!("Path does not exist: '{}'", parm_path.to_str().unwrap());
        return Err(ret)
    }

    // Scenario #1: parm_path is an existing Directory
    if parm_path.exists() && parm_path.is_dir() {
        // Loop through the directory, and build a large JSON.

        for entry in fs::read_dir(parm_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            println!("Found file: {:?}", path);
        }
        return Ok(String::from("Someday will return a JSON from a directory"));
    }

    // Scenario #2: Argument 'parm_path' represents a Single File or Name
    let dpathbuff: DatedPathBuf = DatedPathBuf::new_from_pathbuf(parm_path.to_path_buf())?;
    match dpathbuff.json_metadata() {
        Ok(value) => Ok(value.dump()),
        Err(error) => Err(format!("Received a JSON encoding error: {:?}", error))
    }            
}


pub fn is_path_file8601(path: &Path) -> bool {
    // Checks if a Path contains a valid ISO 8601 datetime string.
    if path.to_str().is_none() {
        return false
    }
    let iso_regex = Regex::new(iso8601_pattern());
    if iso_regex.is_err() {
        return false
    }

    print_path_parts(path);
    iso_regex.unwrap().is_match(path.to_str().unwrap())
}

// ---- PRIVATE FUNCTIONS -----

fn iso8601_pattern() -> &'static str {
    // Return a regular expression for matcing ISO 8601 datetime format.
    // Using the 'concat' macro so I can document the components of the expression.
    concat!(
         "([0-9]{4})"           // year
        ,"-?(1[0-2]|0[1-9])"    // month
        ,"-?(3[01]|0[1-9]|[12][0-9])" // day
        ,"T" // date-time seperator
        ,"(2[0-3]|[01][0-9])"  // hour
        ,":?([0-5][0-9])"  // minute
        ,":?([0-5][0-9])"  // second
        ,"-?(2[0-3]|[01][0-9])" // offset hour
        ,":?([0-5][0-9])"  // offset minute
    )
}


pub fn parse_filename_parts(filename: &str) -> (&str, Option<&str>) {
    // Accept and return references.  No ownership changes.  No Heap strings.
    // Returns 'stem' and 'extension'
    // This is where we handle multiple extensions like '.tar.gz'
    // UTF-8 for a '.' (period, dot, full stop) is 2E
    
    if filename == "." {
        panic!("Invalid file name: single Unicode character U+002E (period, full stop)");
    }

    // Detect a leading period (Unix hidden filename)
    let mut index_start = 0;
    if filename.starts_with('.') {
        index_start = 1;
    }

    // Example of components:  ['some_file_name', 'tar', 'gz']
    let components: Vec<&str> = filename[index_start..].split('.').collect();

    // Scenario 1: Only single component, so there are no extensions.
    if components.len() == 1 && components[0] == &filename[index_start..] {
        return (filename, None)
    }

    // Scenario 2: There exists at least 1 potential extension.
    let mut stop_index: usize = 0;
    for (index, component) in components.iter().enumerate() {

        if index == 0 {
            stop_index = index_start + component.len();
            continue;
        }

        if is_extension_valid(component) {
            // println!("Stem: {}", &filename[0..stop_index]);
            // println!("Extension: {:?}",  Some(&filename[stop_index+1..]));
            return (&filename[0..stop_index], Some(&filename[stop_index+1..]));
        }
        stop_index += component.len();
    };

    (filename, None)
}


fn is_extension_valid(extension: &str) -> bool {
    // Decide whether a string is also a valid Filename extension.
    // For now, we are only rejecting extensions that begin with 0..9
    let first_char: &char= &extension.chars().next().unwrap();

    // Is first character a base 10 digit?
    !first_char.is_ascii_digit()
}

// ----UNIT TESTS----

#[test]
fn filename_components() {
    // To run this test only:  'clear && cargo test -- --nocapture parse_test'
    let filenames = ["some_file_name",
                        ".some_file_name",
                        "some_file_name.gz",
                        ".some_file_name.gz",
                        "some_file_name.tar.gz",
                        "some_file_name_v1.7.5.tar.gz" ];

    let stem_expected = [ "some_file_name",
                          ".some_file_name",
                          "some_file_name",
                          ".some_file_name",
                          "some_file_name",
                          "some_file_name_v1.7.5" ];

    let extension_expected = [ None,
                                None,
                                Some("gz"),
                                Some("gz"),
                                Some("tar.gz"),
                                Some("tar.gz") ];

    for (i, x) in filenames.iter().enumerate() {
        assert_eq!( stem_expected[i], parse_filename_parts(&x).0 );
        assert_eq!( extension_expected[i], parse_filename_parts(&x).1 );
    }
}
