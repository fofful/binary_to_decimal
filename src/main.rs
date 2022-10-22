use std::collections::VecDeque;
use std::env;
use std::process;
use std::char;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_string: String;
    if args.len() == 2{
        input_string = args[1].parse::<String>().unwrap();       
    }
    else{
        println!("No argument given.");
        process::exit(0x0001);
    }
    let mut char_vec: VecDeque<char> = input_string.chars().collect();

    for char in &char_vec{
        if char == &'0' || char == &'1'{} else {
            println!("Invalid input: {char}.");
            process::exit(0x0001);
        }
    }

    let prefix_num = char_vec.pop_front();
    let prefix = if prefix_num.unwrap() == '0'{
        "-"
    } else {
        "+"
    };
    let mut int_sum = 0;
    let mut multiplier = 1;

    while char_vec.len() > 0{
        let char = char_vec.pop_back();
        let num = char.unwrap().to_digit(10).unwrap();
        int_sum = int_sum + num * multiplier;
        multiplier = multiplier * 2;

    }
    print!("{prefix}{int_sum}");
}
