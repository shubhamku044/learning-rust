#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut your_name: String = String::new();

    println!("What is your name: ");

    io::stdin()
        .read_line(&mut your_name)
        .expect("Didn't receive input");

    let greeting: &str = "Nice to meet you";

    println!("{} {}.", greeting, your_name.trim_end());
}
