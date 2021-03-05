#[allow(dead_code)]

use clap::{App, Arg};
use std::fs::File;

mod crc16;
mod cal0;

use crate::cal0::Cal0;

fn main() {
    let args = App::new("CAL0 Checker")
        .author("Madeline S.")
        .arg(
            Arg::with_name("input_file")
                .index(1)
                .required(true)
                .help("Path to a PRODINFO file"),
        )
        .arg(
            Arg::with_name("biskey")
                .long("biskey")
                .short("b")
                .help("The key to use for crypto if the PRODINFO is encrypted"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .default_value("false"),
        )
        .get_matches();

    let fpath = args.value_of("input_file").unwrap();
    let fp = File::open(fpath).expect("Couldn't open file");

    let mut prodinfo = Cal0::from(fp, None).unwrap();

    println!("Prodinfo::[{}]", fpath);

    let cert_is_there = !prodinfo.is_ssl_cert_nulled();
    println!("  [{}] SslCert | {:s02x?}", if cert_is_there { "+" } else { "-" }, prodinfo.header.body_hash);
}
