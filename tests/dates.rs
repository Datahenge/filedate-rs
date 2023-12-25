// dates.rs

/*
These tests helped me understand Chrono.  They are not required in the final
version of FileDate (temporal?  spacetime?)
*/
use chrono::prelude::*;

#[test]
fn test_utc_naive() {
    let dt_naive: NaiveDateTime = NaiveDate::from_ymd(1955, 11, 5).and_hms(20, 10, 5);
    let utc_from_naive: DateTime<Utc> = DateTime::<Utc>::from_utc(dt_naive, Utc);
    assert_eq!(utc_from_naive.to_string(), "1955-11-05 20:10:05 UTC");
}

#[test]
fn test_utc_nooffset() {
    let dt_naive: NaiveDateTime = NaiveDate::from_ymd(1955, 11, 5).and_hms(20, 10, 5);
    let utc_from_naive: DateTime<Utc> = DateTime::<Utc>::from_utc(dt_naive, Utc);
    let no_offset = FixedOffset::east(0);
    let fixed_from_utc = utc_from_naive.with_timezone(&no_offset);
    assert_eq!(fixed_from_utc.to_string(), "1955-11-05 20:10:05 +00:00");
}

#[test]
fn test_utc_offset_west8() {
    let dt_naive: NaiveDateTime = NaiveDate::from_ymd(1955, 11, 5).and_hms(20, 10, 5);
    let utc_from_naive: DateTime<Utc> = DateTime::<Utc>::from_utc(dt_naive, Utc);
    let offset = FixedOffset::west(8 * 3600);
    let fixed_from_utc = utc_from_naive.with_timezone(&offset);
    assert_eq!(fixed_from_utc.to_string(), "1955-11-05 12:10:05 -08:00");
}

#[test]
fn test_zero_offsets() {
    let east_zero = FixedOffset::east(0);
    let west_zero = FixedOffset::west(0);
    assert_eq!(east_zero, west_zero);
}
