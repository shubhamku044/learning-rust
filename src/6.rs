#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let arr_1 = [1, 2, 3, 4];
    let arr_2: [i32; 4] = [1, 2, 3, 4];

    println!("len {}", arr_1.len());

    let mut loop_idx = 0;
    loop {
        if (loop_idx == arr_2.len()) {
            break;
        }
        println!("{}: {}", loop_idx, arr_2[loop_idx]);
        loop_idx += 1;
    }
}
