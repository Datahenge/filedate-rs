// stamp.rs
// This module is about stamping a String (or Path) with an ISO 8601 datetime.

use std::path::{Path, PathBuf};

use super::*;

// 3rd Party
use chrono::prelude::*;
use serde_json::Result as ResultSerde;

static EMPTY_STRING: &str = "";


pub fn stamp_str(parm_file_name: &str, 
	parm_date_time: &Option<DateTime<FixedOffset>>) -> String {

	// Either convert the argument, or get current UTC time.
	let dt_string: String = datetime_to_iso_string(parm_date_time);
	let filename: FileName = FileName::new(parm_file_name);

	if filename.extension.is_none() {
	return format!("{}_{}", filename.stem, dt_string);
	}

	format!("{}_{}.{}",
		filename.stem,
		dt_string,
		filename.extension.unwrap_or(String::from(""))
	)
}

pub fn stamp_path(parm_path: &Path, 
	parm_date_time: &Option<DateTime<FixedOffset>>) -> PathBuf {

	let filename_as_str = parm_path.file_name().unwrap().to_str().unwrap();

	// No parent in the Path.
	if parm_path.parent().is_none() {
		return PathBuf::from(stamp_str(filename_as_str, parm_date_time));
	}

	let parent_path = Path::new(parm_path.parent().unwrap());
	parent_path.join(stamp_str(filename_as_str, parm_date_time))
}

// Private Functions
fn datetime_to_iso_string(parm_date_time: &Option<DateTime<FixedOffset>>) -> String {
	// Returns on owned String, that represents an ISO 8601 datetime value.
	if parm_date_time.is_some() {
		return parm_date_time.unwrap().to_iso_string();
	}
	else {
		return Utc::now().to_iso_string();
	}
}

trait Suffix8601Ext<'a> {
    // Extension Trait! :)
    fn to_iso_string(&self) -> String;
}

impl<'a> Suffix8601Ext<'a> for DateTime<FixedOffset> {
    fn to_iso_string(&self) -> String {
        const ISO_FORMAT: &str = "%Y-%m-%dT%H%M%S%z";
        self.format(ISO_FORMAT).to_string()  // DelayedFormat > String
    }
}

impl<'a> Suffix8601Ext<'a> for DateTime<Utc> {
    fn to_iso_string(&self) -> String {
        // to_rfc3339_opts() returns an owned String
        self.to_rfc3339_opts(SecondsFormat::Secs, false).replace(':',"") 
    }
}


// ----UNIT TESTS----

#[test]
fn test_get_datetime_string() {
    let dt_naive: NaiveDateTime = NaiveDate::from_ymd(1955, 11, 5).and_hms(9, 10, 11);

    let utc_datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(dt_naive, Utc);
    let no_offset = FixedOffset::west(0);
    let utc_minus_0 = utc_datetime.with_timezone(&no_offset);
    assert_eq!( datetime_to_iso_string(&Some(utc_minus_0)), "1955-11-05T091011+0000");

    let offset = FixedOffset::west(8 * 3600);
    let utc_minus_8 = utc_datetime.with_timezone(&offset);
    assert_eq!( datetime_to_iso_string(&Some(utc_minus_8)), "1955-11-05T011011-0800");
}


use serde::Serialize;

// New Struct
#[derive(Serialize)]
pub struct FileName<'a> {
    stem: String,
    extension: Option<String>,
    directory: Option<&'a Path>  // Must be the last field in the Struct, because it has a "dynamically sized type"
}

#[allow(dead_code)]
impl<'a> FileName<'a> {
    pub fn new(some_string: &str) -> FileName {
        let filename_parts: (&str, Option<&str>) = parse_filename_parts(some_string);

        let mut ext: Option<String> = None;
        if filename_parts.1.is_some() {
            ext = Some(filename_parts.1.unwrap().to_owned());
        }

        let filename_new = FileName {
            stem: filename_parts.0.to_owned(),
            extension: ext,
            directory: Path::new(some_string).parent(),
        };
        filename_new
    }

    fn print_as_json(&self) -> () {
        // Serialize it to a JSON string.
        let j: ResultSerde<String> = serde_json::to_string(&self);

        if j.is_ok() {
            println!("{}", j.unwrap());
        }
    }

    pub fn to_owned_str(&self) -> String {
        // Ideally I'd prefer to return a borrowed string.
        // I don't feel we're actually "creating" any new data.
        // But for the moment, all contact/join functions will only return owned strings.
        let y: &str = match &self.extension {
            Some(value) => value,
            None => EMPTY_STRING
        };
        [&self.stem, y].join(".")
     }
}


#[test]
fn filename_to_string() {
    let some_file_name: FileName = FileName::new("some_file_name.tar.gz");
    assert_eq!("some_file_name.tar.gz", some_file_name.to_owned_str());
}
