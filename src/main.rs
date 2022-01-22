mod profile;

use profile::generic::ColorSchemes;
use profile::generic::SchemeError;
use std::{fs, io};
use std::fmt::Debug;
use chardetng::EncodingDetector;
use encoding_rs::{Encoding};
use clap::{App, Arg};
use std::io::{Read, stderr, Write};

#[derive(Debug)]
pub enum SchemeFormat {
    WindowsTerminal,
    SecureCRT,
    XShell,
    Alacritty,
    MobaXTerm,
}

impl SchemeFormat {
    fn from_str(s: &str) -> Result<SchemeFormat, SchemeError> {
        match s.to_lowercase().trim() {
            "alacritty" => Ok(SchemeFormat::Alacritty),
            "xshell" | "xcs" => Ok(SchemeFormat::XShell),
            "windowsterminal" | "wt" => Ok(SchemeFormat::WindowsTerminal),
            // "mobaxterm" => Ok(SchemeFormat::MobaXTerm),
            _ => Err(SchemeError::Unsupported)
        }
    }
}


fn main() {
    // Option parsing
    let matches = App::new("TCconv")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("from")
            .short('f')
            .long("from")
            .value_name("FROM_FORMAT")
            .takes_value(true)
            .help("From format. Case insensitive (eg. wt)")
        )
        .arg(Arg::new("to")
            .short('t').long("to")
            .value_name("TO_FORMAT")
            .takes_value(true)
            .help("To format. Case insensitive (eg. alacritty)")
        )
        .arg(Arg::new("INPUT_FILE")
            .help("Source scheme file")
        )
        .arg(Arg::new("list")
            .short('l').long("list")
            .takes_value(false)
        )
        .arg(Arg::new("OUTPUT_FILE")
            .short('o').long("output")
            .takes_value(true)
            .help("Target scheme file")
        )
        .get_matches();

    match matches.occurrences_of("list") {
        0 => {}
        _ => {
            list_available_formats();
            return;
        }
    };
    if matches.value_of("from").is_none() || matches.value_of("to").is_none() {
        print_usage();
        std::process::exit(-1);
    }
    let scheme_from = matches.value_of("from").unwrap();
    let scheme_to = matches.value_of("to").unwrap();
    let file_name = matches.value_of("INPUT_FILE");
    let output_file = matches.value_of("OUTPUT_FILE");

    let scheme_from = SchemeFormat::from_str(scheme_from).unwrap();
    let scheme_to = SchemeFormat::from_str(scheme_to).unwrap();
    let src = match file_name {
        // From file
        Some(name) => { fs::read(name) }
        // From stdin
        None => { io::stdin().bytes().collect() }
    }.unwrap();

    let result = convert(src.as_slice(), scheme_from, scheme_to);
    match output_file {
        Some(name) => {
            let mut file = fs::File::create(name).unwrap();
            file.write_all(result.as_bytes()).unwrap();
        }
        None => {
            io::stdout().write_all(result.as_bytes()).unwrap();
        }
    }
}

fn print_usage() {
    stderr().write_all(b"-h for usage\n").unwrap();
}

fn convert(input: &[u8], scheme_from: SchemeFormat, scheme_to: SchemeFormat) -> Box<String> {

    // Guess encoding.
    let encoding = guess_encoding(input);

    // Decode into str with giving up unrecognized bytes
    let (input, _, err) = encoding.decode(input);
    if err {
        panic!("Unrecognized format");
    };
    // Convert str to ColorSchemes
    let gcs = ColorSchemes::from_literal(input.as_ref(), scheme_from);
    // ColorSchemes to str
    gcs.unwrap().to_literal(scheme_to)
}

fn list_available_formats() {
    io::stdout().write_all(b"wt,windows terminal,windowsterminal
xcs,xshell
alacritty").unwrap();
}


#[test]
fn test_ground() {
    let f = std::fs::File.open("test/alacritty.yml").unwrap();
    let mut reader = io::BufReader::new(f);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
}

fn guess_encoding(buf: &[u8]) -> &'static Encoding {
    let mut det = EncodingDetector::new();
    det.feed(buf.as_ref(), true);
    det.guess(None, true)
}