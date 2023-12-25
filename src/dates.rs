// dates.rs
use std::path::Path;

// Third Party
use chrono::prelude::*;
use regex::Regex;

const ISO_FORMAT: &'static str = "%Y-%m-%dT%H%M%S%z";

// Created an 'Extension Trait', that teaches DateTime types how to become iso_strings! :)
trait Suffix8601Ext<'a> {
    // From what I can tell, there's no way to generated a &str from a DateTime.
    // This has a major downstream consequence: our Structs cannot be entirely borrowed from a &Path and &DateTime.
    fn to_iso_string(&self) -> String;
    // fn to_borrowed_iso_str(&self) -> &str;
}

impl<'a> Suffix8601Ext<'a> for DateTime<FixedOffset> {

    fn to_iso_string(&self) -> String {
        self.format(ISO_FORMAT).to_string()  // DelayedFormat > String
    }
}
 
impl<'a> Suffix8601Ext<'a> for DateTime<Utc> {
    fn to_iso_string(&self) -> String {
        // to_rfc3339_opts() returns an owned String
        self.to_rfc3339_opts(SecondsFormat::Secs, false).replace(":","") 
    }
}


pub fn is_path_datetime_stamped<S>(path: S) -> bool
    where S: AsRef<Path> {
    // Checks if a Path contains a valid ISO 8601 datetime string.
    let iso_regex = Regex::new(iso8601_pattern());
    if iso_regex.is_err() {
        return false
    }
    iso_regex.unwrap().is_match(path.as_ref().to_str().unwrap())
}

pub fn path_datetime_indices(path: &Path) -> Option<(usize, usize)> {

    // Checks if the Path contains a valid ISO 8601 datetime string.
    let iso_regex = Regex::new(iso8601_pattern());
    if iso_regex.is_err() {
        return None
    }
    let match_loc = iso_regex.unwrap().find(path.to_str().unwrap());
    if match_loc.is_none() {
        return None;
    }
    Some((
        match_loc.unwrap().start(), match_loc.unwrap().end()
    ))
}


pub fn datetime_from_iso_string(dt_as_string: &str) -> Option<DateTime<FixedOffset>> {
    const FMT: &str = "%Y-%m-%dT%H%M%S%z";
    match DateTime::parse_from_str(dt_as_string, FMT) {
        #[allow(unused_variables)]
        Err(e) => {
            return None;
        }
        Ok(f) => {
            return Some(f);
        }                
    };
}


pub fn datetime_to_iso_string(parm_date_time: &Option<DateTime<FixedOffset>>) -> String {
	// Returns on owned String, that represents an ISO 8601 datetime value.
	if parm_date_time.is_some() {
		parm_date_time.unwrap().to_iso_string();
	}
	else {
		Utc::now().to_iso_string();
	}        
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


// ----UNIT TESTS----

#[test]
fn test_get_datetime_string() {
    
    let dt_naive: NaiveDateTime = NaiveDate::from_ymd(1955, 11, 5).and_hms(9, 10, 11);

    fn dtnaive_to_dtfixed(dt_naive: NaiveDateTime,
                                 tz_offset: FixedOffset) -> DateTime<FixedOffset> {
        tz_offset.from_local_datetime(&dt_naive).unwrap()
    }

    // Scenario 1
    {
        let no_offset = FixedOffset::west(0);
        let dt_fixed: DateTime<FixedOffset> = dtnaive_to_dtfixed(dt_naive, no_offset);
        assert_eq!( datetime_to_iso_string(&Some(dt_fixed)), "1955-11-05T091011+0000");
    }

    // Scenario 2
    {
        let offset = FixedOffset::west(8 * 3600);
        let dt_fixed: DateTime<FixedOffset> = dtnaive_to_dtfixed(dt_naive, offset);
        assert_eq!( datetime_to_iso_string(&Some(dt_fixed)), "1955-11-05T091011+0000");
    }

    // Scenario 3
    {
        let utc_datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(dt_naive, Utc);
        let no_offset = FixedOffset::west(0);
        let utc_minus_0 = utc_datetime.with_timezone(&no_offset);
        assert_eq!( datetime_to_iso_string(&Some(utc_minus_0)), "1955-11-05T091011+0000");
    }

    // Scenario 4
    {
        let utc_datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(dt_naive, Utc);
        let offset = FixedOffset::west(8 * 3600);
        let utc_minus_8 = utc_datetime.with_timezone(&offset);
        assert_eq!( self::datetime_to_iso_string(&Some(utc_minus_8)), "1955-11-05T011011-0800");
    }        
}
