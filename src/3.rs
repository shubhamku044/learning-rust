#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    // unsigned int - u8, u16, u32, u64, u128, usize
    // signed int - i8, i16, i32, i64, i128, isize

    println!("max u32 {}", u32::MAX);
    println!("max u64 {}", u64::MAX);
    println!("max u128 {}", u128::MAX);
    println!("max usize {}", usize::MAX);
    println!("max i32 {}", i32::MAX);
    println!("max i64 {}", i64::MAX);
    println!("max isize {}", isize::MAX);
    println!("max f32 {}", f32::MAX);
    println!("max f64 {}", f64::MAX);

    let my_bool: bool = true;

    let num_1: f32 = 1.11111111;
}
