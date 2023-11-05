#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let str1 = String::from("Hello");
    // let str2 = str1;
    let str2 = str1.clone();

    println!("{} Wrold", str1);
}
