// bin.rs
// Standard Library
use std::path::Path;

// Third Party
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};  //, ArgMatches};

// Internals
// pub mod stamp;

// filedate::filename_to_json(&args[1]);

// Private function for the CLI, which receives dates as Strings.
fn cli_add_suffix(filename_str: &str, date_str: &str) {
    match  filedate::datetime_from_string(date_str) {
        None => {
            println!("Invalid datetime string.");
            std::process::exit(0);
        }
        Some(f) => {
            let new_filename: String = filedate::stamp::stamp_str(filename_str, &Some(f));
            println!("{}", new_filename);  //output the result
        }                
    };
}

fn cli_add_suffix_no_date(filename_orig: &str) {
    let new_filename: String = filedate::stamp::stamp_str(filename_orig, &None);
    println!("{}", new_filename);  //output the result to terminal
}

fn add_arguments<'a, 'b>(cli_app: App<'a, 'b>) -> App<'a, 'b> {
    // This function adds arguments and subcommands to a Clap App.

    // Achieving this was trickier than I expected:
    //   1) App has 2 lifetimes, which I had to explicitly name.
    //   2) Methods like arg() and subcommand() take ownership.
    //      So either you chain everything in 1 pass.  Or you use a variable let 'ret' to keep capturing ownership.

    // Add some arguments.    
    let ret = cli_app
        .arg(
            Arg::with_name("debug")
            .help("turn on debugging information")
            .short("d")
        );

    // Add some subcommands.
    let ret = ret
        .subcommand(SubCommand::with_name("stamp")
            .about("stamps a filename with an ISO 8601 datetime")
            .arg(Arg::with_name("filename")
                .help("The filename to stamp")
                .required(true),
            )
            .arg(Arg::with_name("datetime")
                .help("A datetime string")
                .required(false),
            )      
        )
        .subcommand(SubCommand::with_name("extract")
            .about("extract metadata from a path or filename")
            .arg(Arg::with_name("path")
                .help("The path to a file or directory")
                .required(true)
            )
            .arg(Arg::with_name("check")
                .short("c")
                .long("--check")
                .help("verify that path exists")
                .takes_value(false),
            )
        );
    ret
}

fn main() {

    // I could have chained arguments and subcommands here, but wanted to keep main() clean and tidy.
    let cli_app = add_arguments(
        App::new("filedate")
        .about("CLI for filedate")
        .version(filedate::get_package_version())  // altnerately, .version(crate_version!())
        .author("Brian Pond <brian@pondconsulting.net>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
    );

    // Method get_matches() takes ownership of a clap App, and  returns a ArgMatches.  Effectively destroying App!
    // Having read the Clap comments, this is what the developer intended.
    let matches = cli_app.get_matches();

    match matches.subcommand() {
        ("stamp", Some(stamp_matches)) => {
            let filename = stamp_matches.value_of("filename").unwrap();
            if stamp_matches.is_present("datetime") {
                cli_add_suffix(filename, stamp_matches.value_of("datetime").unwrap());
            } else {
                cli_add_suffix_no_date(filename);
            }
        },
        ("extract", Some(extract_matches)) => {
            let path = Path::new(extract_matches.value_of("path").unwrap());
            let check : bool = extract_matches.is_present("check");
            let ret = filedate::build_metadata_from_path(path, check);
            std::process::exit(match ret {
                Ok(_) => {
                    println!("{:?}", ret.unwrap());
                    0
                },
                Err(err) => {
                    eprintln!("error: {:?}", err);
                    1
                }
            });
            
        },
        ("", None) => println!("Please specify a subcommand (stamp, extract)"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
