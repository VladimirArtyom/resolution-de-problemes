use std::{collections::HashMap, io};
pub fn cf_watermelon() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please type a number");
    if number % 2  == 0  && number > 2{
        println!("YES");
    } else {
        println!("NO");
    }
}

pub fn cf_word_capitalization() {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let first_char = input.chars().next().unwrap().to_uppercase().to_string();
    
    println!("{}{}", first_char, input.chars().skip(1).collect::<String>() )
}

pub fn cf_boy_or_girl() {
    let mut input: String = String::new();
    let mut hm: HashMap<char, i32> = HashMap::new();
    let mut answer : i32 = 0; // because the null probably, i dont know

    io::stdin().read_line(&mut input).expect("Failed to read the line");
    
    for c in input.chars() {
        if !hm.contains_key(&c) {
            hm.insert(c, 1);    
            answer += 1;
        }
    }

    if answer % 2 == 0 {
        
        println!("{}", "CHAT WITH HER!");
    }
    else {
        println!("{}", "IGNORE HIM!");

    }

}

pub fn bear_and_big_brother() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed bng");

}
