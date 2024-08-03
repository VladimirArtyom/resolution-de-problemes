use std::{collections::HashMap, io::{self, Read}, str::Chars};
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
    let mut year: i32 = 0; 
    io::stdin().read_line(&mut input).expect("failed bng");

    let parts: Vec<&str> = input.split_whitespace().collect(); 

    let mut a = parts[0].parse::<i32>().expect("failed to parse");
    let mut b = parts[1].parse::<i32>().expect("failed to parse");

    while a <= b{
        a *=3;
        b *=2;
        year += 1;
    }

    println!("{}", year);
}

pub fn stones_on_the_table() {
    let mut input = String::new();
    let mut stones = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    io::stdin().read_line(&mut stones).expect("Failed to read line");

    let mut answer: i32 = 0;
    for indx in 0..stones.len() - 1 {
        if stones.chars().nth(indx) == stones.chars().nth(indx + 1) {
            answer +=1;
        }
    }
    println!("{}", answer)
}

pub fn dp_stones_on_the_table() {
    let mut total = String::new();
    let mut stones_str = String::new();

    io::stdin().read_line(&mut total).expect("cannot read line");
    io::stdin().read_line(&mut stones_str).expect("cannot read line");

    let stones = stones_str.trim().as_bytes();
    let mut dp = vec![0; stones.len()];
    for indx in 1..stones.len(){
        dp[indx] =  dp[indx - 1];
        if stones[indx] == stones[indx-1] {
            dp[indx] += 1;
        }
    }

    println!("{:?}", dp[stones.len() - 1]);
}


pub fn dp_elephant() {

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
    println!("{}", dp.len());
}

pub fn cf_nearly_lucky_number() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut int_input: u64 = input.trim().parse::<u64>().unwrap();
    let mut ans: i8 = 0;
    while int_input != 0{
        let value: u64 = int_input % 10;
        if value == 4 {
            ans += 1;
        }

        if value == 7 {
            ans += 1
        }
        int_input = int_input / 10;
    }

    if ans == 4 || ans == 7{
        println!("YES");
    } else {
        println!("NO");
    }
}

pub fn cf_wrong_subtraction() {
    
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    let inputs = input.split(" ").collect::<Vec<&str>>();

    let mut num: i32 = inputs[0].trim().parse::<i32>().unwrap();
    let step: i32 = inputs[1].trim().parse::<i32>().unwrap();
    
    for _ in 0..step {
        if num % 10 == 0{
            num = num / 10;
        } else {
            num = num - 1;
        }
    }

    println!("{}", num);
    
}

pub fn cf_anton_and_danik() {
    
    let mut input_1 : String = String::new();
    let mut input_2: String = String::new();
    
    io::stdin().read_line(&mut input_1).expect("Failed to read line");
    io::stdin().read_line(&mut input_2).expect("Failed to read line");

    let mut int_input: i32 = input_1.trim().parse().unwrap() ;
    let char_2_input: std::str::Chars = input_2.chars();

    let mut ada: i32 = 0;
    let mut dan: i32 = 0;

    for ch in char_2_input{
        if ch == 'A' {
            ada += 1;
        }
        else if ch == 'D' {
            dan += 1;
        } 
    }
        
    if ada > dan {
        println!("Anton");
    } else if ada < dan {
        println!("Danik");
    } else {
        println!("Friendship");
    }

}

pub fn cf_divisibility_problem() {
    let mut main_input: String = String::new();
    io::stdin().read_line(&mut main_input).expect("No you can't");
    
    let loop_main: i64 = main_input.trim().parse().unwrap();
    let mut vec_ans: Vec<i32> = vec![];
    for _ in 0..loop_main {
        let mut input_1: String = String::new();

        io::stdin().read_line(&mut input_1).expect("Failed to read line");

        let vec = input_1.trim().split(" ").collect::<Vec<&str>>();
        let int_input_1: i32 = vec[0].trim().parse::<i32>().unwrap();
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

pub fn cf_policeman() {

    let mut input: String = String::new();
    let mut input_2: String = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    io::stdin().read_line(&mut input_2)
        .expect("Failed to read line");

    let vec_input_2: Vec<i32>= input_2.split(" ").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let mut available_officers: i32 = 0;
    let mut available_crimes: i32 = 0;


    for event in vec_input_2 {
        if event == -1 { 
            if available_officers > 0 {
                available_officers -= 1;
            }
            else {
                available_crimes +=1;
            }
        }
        else {
            available_officers += event;
        }
    }


    println!("{}", available_crimes);

}

pub fn sum_of_round_numbers() {
    
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please type a number");

    for _ in 0..number {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut num = input.trim().parse::<i32>().unwrap();


            let mut ve: Vec<i32> = Vec::new();
            let mut multiplier: i32 = 1;
            while num > 0{

                let num_dev = num % 10;
                if num_dev != 0 {
                    ve.push(num_dev * multiplier);
                }
                multiplier *= 10;
                num = num / 10;
            }
            println!("{}", ve.len());
            for i in ve {
                print!("{} ", i);
            }
    }
}


pub fn cf_soldier_and_bananas() { 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");
    
    let veca: Vec<i32> = input.trim().split(" ").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    
    let (k, n, w) = (veca[0], veca[1], veca[2]);
    
    
    let mut total: i32= 0; 
    for i in 1..=w{
        total += i * k;
    }

    if total - n < 0 {
        println!("{}", 0);
        return;
    }

    total = total -  n;

    println!("{}", total);

}


pub fn cf_translation() {
    
}
