#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u32};

use std::collections::HashMap;

fn main() {
    const PI: f32 = 3.14;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rect {
        length: f32,
        width: f32,
    }
    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rect {
        fn new(length: f32, width: f32) -> Rect {
            return Rect { length, width };
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    };

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rect = Shape::new(10.0, 10.0);
    let cir: Circle = Shape::new(10.0, 10.0);

    println!("Rectangle area: {}", rec.area());
    println!("Circle area: {}", cir.area())
}
