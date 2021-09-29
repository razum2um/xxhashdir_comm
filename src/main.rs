use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn parse_line(s: String) -> (u64, String) {
    let xxhash = &s[0..20].trim().parse::<u64>().unwrap();
    let path = String::from(&s[22..]);
    (*xxhash, path)
}

// FIXME: make main iteration using `for_each_parsed_line(|(h, p2)|)`
// see https://github.com/lycheeverse/lychee/pull/339/files also to handle pipes

// fn for_each_parsed_line(f: File) -> ? {
//     BufReader::new(f).lines().filter_map(|line| line.ok()).map(parse_line)
// }

// fn paths_by_xxhash(f: File) -> HashMap<u64, String> {
//     for_each_parsed_line(f).collect()
// }

const BLANK_FILE: u64 = 17241709254077376921;
const DS_STORE_FILE: u64 = 4404938841486182190;

fn paths_by_xxhash(f: File) -> HashMap<u64, String> {
    BufReader::new(f)
        .lines()
        .filter_map(|line| line.ok())
        .map(parse_line)
        .collect()
}

fn main() -> io::Result<()> {
    let matches = App::new("xxhashdir_comm")
        .version("1.0")
        .about("Takes 2 files from xxhashdir out and finds common or distinct entries.")
        .arg(
            Arg::new("common")
                .short('c')
                .long("common")
                .conflicts_with("only-second")
                .about("Output only lines with hashes in both files"),
        )
        .arg(
            Arg::new("only-second")
                .short('2')
                .long("only-second")
                .conflicts_with("common")
                .about(
                "Output only lines from second file which not present in the first (by default)",
            ),
        )
        .arg(
            Arg::new("respect-empty")
                .long("respect-empty")
                .about("Don't skip empty files (they all has same hash)"),
        )
        .arg(
            Arg::new("respect-ds-store")
                .long("respect-ds-store")
                .about("Don't .DS_Store files (they all has same hash)"),
        )
        .arg(Arg::new("FILE1").required(true).takes_value(true))
        .arg(Arg::new("FILE2").required(true).takes_value(true))
        .get_matches();

    let common = matches.is_present("common");
    let respect_empty = matches.is_present("respect-empty");
    let respect_ds_store = matches.is_present("respect-ds-store");
    let only_second = matches.is_present("only-second") || !common;

    let f = File::open(&matches.value_of("FILE1").unwrap())?;
    let parsed_f1 = paths_by_xxhash(f);

    let f2 = File::open(&matches.value_of("FILE2").unwrap())?;
    BufReader::new(f2)
        .lines()
        .filter_map(|line| line.ok())
        .map(parse_line)
        .for_each(|(h, p2)| {
            if ((h != BLANK_FILE) || respect_empty) && ((h != DS_STORE_FILE) || respect_ds_store) {
                if let Some(p1) = parsed_f1.get(&h) {
                    if common {
                        if p1 == &p2 {
                            println!("{:<21} {}", h, p2);
                        } else {
                            println!("{:<21} {} {}", h, p1, p2);
                        }
                    }
                } else if only_second {
                    println!("{:<21} {}", h, p2);
                }
            }
        });

    Ok(())
}
