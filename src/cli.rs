use clap::{value_parser, Arg, ArgAction, Command};
use std::env;

use crate::types;

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
            Arg::new("width")
                .short('w')
                .long("width")
                .required(false)
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .help("Width of the (E)DT"),
        )
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .action(clap::ArgAction::SetTrue)
                .help("generate a linear genome and exit"),
        )
        .arg(
            Arg::new("inelastic")
                .short('i')
                .long("inelastic")
                .action(clap::ArgAction::SetTrue)
                .help("output inelastic degenerate string"),
        )
        .arg(
            Arg::new("d")
                .short('d')
                .long("percent-degenerate")
                .default_value("10.0")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set)
                .help("percentage of degenerate loci"),
        )
        .arg(
            Arg::new("fasta")
                .short('f')
                .long("fasta")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(false)
                .help("input fasta file. If multifasta, it uses only the first sequence"),
        )
        .arg(
            Arg::new("max_length")
                .short('l')
                .long("max-length")
                .default_value("1")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .help("max length of a degenerate segment"),
        )
        .arg(
            Arg::new("max_variants")
                .short('s')
                .long("max-variants")
                .default_value("2")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .help("Maximum number of variants in a degenerate position"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let fasta: Option<String> = matches.get_one::<String>("fasta").map(|s| String::from(s));
    let n: Option<usize> = matches.get_one::<usize>("width").copied();
    let g: bool = *matches.get_one::<bool>("generate").unwrap();
    let i: bool = *matches.get_one::<bool>("inelastic").unwrap();
    let d: f64 = *matches.get_one::<f64>("d").unwrap();
    let s: usize = *matches.get_one::<usize>("max_variants").unwrap();
    let l: usize = *matches.get_one::<usize>("max_length").unwrap();

    if fasta.is_none() && n.is_none() {
        panic!("[simed::cli] Expect either width or fasta");
    }

    types::Config {
        fasta,
        i,
        g,
        n,
        d,
        s,
        l,
        max_degenerate: None,
    }
}
