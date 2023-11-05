#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    let age: i32 = 8;

    if (age >= 1) && (age <= 18) {
        println!("Do something");
    } else if (age == 1) {
        println!("Do something");
    } else {
        println!("Do something");
    }

    let my_age: i32 = 69;

    let can_vote: bool = if my_age >= 18 { true } else { false };

    match my_age {
        1..=18 => println!("Imp birthday"),
        21 | 50 => println!("imp "),
        65..=i32::MAX => println!("abc"),
        _ => println!("Not imp"),
    };

    let voting_age: i32 = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("abc"),
        Ordering::Greater => println!("abc"),
        Ordering::Equal => println!("abc"),
    };
}
