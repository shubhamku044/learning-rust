#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

use std::ops::Add;

fn get_sum<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    let num = get_sum(6, 9);
    println!("{}", num);
}
