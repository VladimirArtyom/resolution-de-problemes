mod cf_solves;

use std::fmt::write;
use std::io::Read;
use std::{collections::HashMap, io};
use std::{f64::consts::LN_2};

fn main() {

    let mut main_input: String = String::new();
    io::stdin().read_line(&mut main_input).expect("No you can't");
    
    let loop_main: i64 = main_input.trim().parse().unwrap();
    let mut vec_ans: Vec<i32> = vec![];
    for _ in 0..loop_main {
        let mut input_1: String = String::new();

        io::stdin().read_line(&mut input_1).expect("Failed to read line");

        let vec = input_1.trim().split(" ").collect::<Vec<&str>>();
        let mut int_input_1: i32 = vec[0].trim().parse::<i32>().unwrap();
        let int_input_2: i32 = vec[1].trim().parse::<i32>().unwrap();
        
        if int_input_1 % int_input_2 == 0 {
            vec_ans.push(0);
        } else {
            let remainder : i32 = int_input_1 % int_input_2;
            vec_ans.push(int_input_2 - remainder);
        }
    }

    for i in vec_ans {
        println!("{}", i);
    }

}
