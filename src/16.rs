#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

use std::collections::HashMap;

fn main() {
    let mut heroes: HashMap<&str, &str> = HashMap::new();

    heroes.insert("Superman", "Clark kent");
    heroes.insert("Batman", "Bruce Wayne");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");

        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Not a hero"),
        }
    }
}
