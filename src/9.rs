#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let st3 = String::from("x r t b h k a m c");
    let mut v1: Vec<char> = st3.chars().collect();

    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }
}
