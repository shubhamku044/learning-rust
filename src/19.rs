#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    order_food();
}
