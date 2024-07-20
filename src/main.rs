mod cf_solves;

use std::io::Read;
use std::{collections::HashMap, io};
use std::{f64::consts::LN_2};
fn main() {
        let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut numbers = input.trim().split_whitespace();
    let a: f64 = numbers.next().unwrap().parse().unwrap();
    let b: f64 = numbers.next().unwrap().parse().unwrap();

    // Calculate the minimum y such that Limak's weight exceeds Bob's weight
    let y = ((b.log(10.0) - a.log(10.0)) / (LN_2 - LN_2)).ceil() as i32;

    println!("{}", y);
}
