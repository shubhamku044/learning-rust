#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101); // 1 to 100

    println!("Random number {}", random_num);
}
