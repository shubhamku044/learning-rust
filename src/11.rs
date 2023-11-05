#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

fn main() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("1"),
        Days::Tuesday => println!("2"),
        Days::Wednesday => println!("3"),
        Days::Thursday => println!("4"),
        Days::Friday => println!("5"),
        Days::Saturday => println!("6"),
        Days::Sunday => println!("7"),
    }

    println!("Is today the weekend {}", today.is_weekend());
}
