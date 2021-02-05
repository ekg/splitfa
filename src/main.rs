#![allow(clippy::too_many_arguments)]

use std::io::{self};
use seq_io::fasta::{Reader, Record};
//use substring::Substring;
use std::str;

extern crate clap;
use clap::{App, Arg};

fn split_fasta(input: &str, seg_length: usize, step: usize) {
    let mut reader = Reader::from_path(input).unwrap();
    while let Some(result) = reader.next() {
        let record = result.unwrap();
        let seq = record.full_seq();
        let name = record.id().unwrap();
        let mut start: usize = 0;
        let total_length: usize = seq.len();
        while start + seg_length <= total_length {
            println!(">{}:{}-{}", name, start, start + seg_length);
            println!("{}", str::from_utf8(&seq[start..(start + seg_length)]).unwrap());
            start += step;
        }
        if start < total_length {
            start = total_length - seg_length;
            println!(">{}:{}-{}", name, start, start + seg_length);
            println!("{}", str::from_utf8(&seq[start..(start + seg_length)]).unwrap());
        }
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("splitfa")
        .version("0.1.0")
        .author("Erik Garrison <erik.garrison@gmail.com>")
        .about("Split a FASTA file into subsequences of a given length and overlap length")
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("input FASTA file"),
        )
        .arg(
            Arg::with_name("seg-length")
                .short("l")
                .long("seg-length")
                .takes_value(true)
                .help("Length of the splits"),
        )
        .arg(
            Arg::with_name("step")
                .short("s")
                .long("step")
                .takes_value(true)
                .help("Step size between splits"),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();

    let seg_length = matches.value_of("seg-length").unwrap().parse::<usize>().unwrap();

    let step = matches.value_of("step").unwrap().parse::<usize>().unwrap();

    split_fasta(filename, seg_length, step);

    Ok(())
}
