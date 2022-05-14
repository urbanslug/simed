use clap::{Arg, Command};
use std::env;

use crate::types;
use crate::utils;

// Env vars
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn start() -> types::Config {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::new("small_n")
                .required(true)
                .takes_value(true)
                .help("Value for small n"),
        )
        .arg(
            Arg::new("d")
                .short('d')
                .long("percent-degenerate")
                .multiple_values(false)
                .default_value("10")
                .help("percentage of degenerate loci"),
        )
        .arg(
            Arg::new("max_length")
                .short('l')
                .long("max-length")
                .multiple_values(false)
                .default_value("1")
                .help("max length of a degenerate segment"),
        )
        .arg(
            Arg::new("max_variants")
                .short('s')
                .long("max-variants")
                .multiple_values(false)
                .default_value("2")
                .help("Maximum number of variants in a degenerate position"),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .help("Sets the level of verbosity [default: 0]"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let n: usize = matches
        .value_of("small_n")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let d: usize = matches.value_of("d").unwrap().parse::<usize>().unwrap();
    let s: usize = matches
        .value_of("max_variants")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let l: usize = matches
        .value_of("max_length")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let x = utils::percent(d, n);
    types::Config {
        n,
        d,
        s,
        l,
        max_degenerate: x,
    }
}
