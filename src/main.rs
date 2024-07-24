mod cf_solves;

use std::io::Read;
use std::{collections::HashMap, io};
use std::{f64::consts::LN_2};

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Faild to read line");
    
    let mut x: usize = input.trim().parse::<usize>().unwrap();

    let mut dp: Vec<usize> = Vec::new();
    let steps= vec![5, 4, 3, 2, 1];
    let mut trace: usize = 1;
    dp.push(0);
    let mut x_int: i32 = x as i32;
    while  x_int > 0 {
        for step in steps.iter(){
            x_int = x_int - step;
            if x_int > 0 {
                dp.push(dp[trace - 1] + *step as usize);
                trace += 1;
                break;
            } else {
                break;
            }
        }
    }
    println!("{:?}", dp);
    println!("{}", dp.len());
}
