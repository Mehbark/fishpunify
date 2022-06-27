use itertools::Itertools;

use std::env;
use std::io::{self, BufRead};

mod parse;

use parse::{punify, puns};

fn main() {
    let mut args = env::args();
    let first_arg = args.nth(1).map(|s| s.trim().to_string());

    match first_arg.as_deref() {
        Some("-h") | Some("--help") => help(),
        Some("--kelp") => {
            eprintln!("hi cool :) that's cool! good cool thank you fun pun yayy");
            eprintln!("fish");
            help();
        }
        Some(_) => from_args(),
        None => from_stdin(),
    }
}

fn help() {
    eprintln!("Usage: fishpunify [-h|--help] TEXT...");
    eprintln!("  -h, --help  print this help information");
    eprintln!();
    eprintln!("Will punify TEXT if provided, otherwise will punify linewise from stdin.");
    eprintln!("TEXT will be joined together with single spaces (` `).");
}

fn from_stdin() {
    let puns = puns();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from stdin :(((((");
        let punified = punify(&puns, &line);
        println!("{punified}");
    }
}

fn from_args() {
    let puns = puns();

    let input = env::args().skip(1).join(" ");
    let punified = punify(&puns, &input);

    println!("{punified}");
}
