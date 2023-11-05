#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let my_tuple: (u8, String, f64) = (69, "shubham".to_string(), 50_000.00);

    println!("Name: {}", my_tuple.1);
}
