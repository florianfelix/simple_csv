use clap::Parser;
use clio::*;
// use std::io::Write;

#[derive(Parser, Debug)]

pub struct Cli {
    pub path: Option<ClioPath>,
    #[clap(value_parser, default_value = ";")]
    pub delim: char,
}
