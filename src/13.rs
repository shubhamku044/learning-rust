#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn get_sum(x: i32, y: i32) -> i32 {
    println!("{}", x + y);
    return x + y;
}

fn main() {
    let num: i32 = get_sum(6, 9);
}
