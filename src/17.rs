#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

use std::collections::HashMap;

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };

    bob.address = String::from("553 another st");

    struct Rect<T, U> {
        length: T,
        height: U,
    }

    let rect = Rect {
        length: 4,
        height: 10.5,
    };
}
