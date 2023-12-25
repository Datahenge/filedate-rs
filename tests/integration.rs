// integration.rs

// Standard
use std::path::{Path};

// Third Party
use chrono::prelude::*;

use filedate;

// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
// You can enable stdout with command syntax 'cargo test -- --nocapture'
    

#[test]
#[should_panic]
fn test_std_path_library() {
    // Demonstrate that Path library does not understand chained file extensions.
    // Path will return extension 'gz', instead of 'tar.gz'
    let filename: &str = "myfile.tar.gz";
    assert_eq!(Path::new(&filename).extension().unwrap() ,"tar.gz" );
}

#[test]
fn suffix_test() {
    const DATETIME_STRING: &str = "1955-11-05T011011-0800";
    let some_datetime: Option<DateTime<FixedOffset>> = filedate::datetime_from_string(DATETIME_STRING);
    let filenames = ["some_file_name",
                        ".some_file_name",
                        "some_file_name.gz",
                        ".some_file_name.gz",
                        "some_file_name.tar.gz" ];

    // Vectors must have same type; a good thing when doing comparision loops like this.
    let expected = [ "some_file_name_1955-11-05T011011-0800",
                        ".some_file_name_1955-11-05T011011-0800",
                        "some_file_name_1955-11-05T011011-0800.gz",
                        ".some_file_name_1955-11-05T011011-0800.gz",
                        "some_file_name_1955-11-05T011011-0800.tar.gz"];
                    
    for (i, x) in filenames.iter().enumerate() {
        assert_eq!( expected[i], filedate::stamp::stamp_str(&x, &some_datetime));
    }
}
