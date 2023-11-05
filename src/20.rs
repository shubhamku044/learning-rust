#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let path = "lines.txt";

    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };

    write!(output, "Just some\nRandom words").expect("Failed to write in the file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}
