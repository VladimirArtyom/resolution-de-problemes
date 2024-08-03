mod cf_solves;

use std::fmt::write;
use std::io;
fn main() {

    let mut string_1: String = String::new();
    let mut string_2: String =String::new();

    io::stdin().read_line(&mut string_1).expect("failed to read line");
    io::stdin().read_line(&mut string_2).expect("failed to read line");

    let length = string_1.trim().chars().count();
    let length_2 = string_2.trim().chars().count();
    if length != length_2 {

        println!("NO");
        return;
    }
    for index in 0..length  {    
        if string_1.chars().nth(index).unwrap() != string_2.chars().nth(length - index - 1).unwrap() {
            println!("NO");
            return;
        }
    }
    println!("YES");
    return

}


