use core::panic;
use std::env::{args, Args};

fn main() -> () {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_as_num = first.parse::<f32>().unwrap();
    let second_as_num = second.parse::<f32>().unwrap();

    let result = operate(operator, first_as_num, second_as_num);

    println!("{}", output(first_as_num, operator, second_as_num, result))
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '/' => first_num / second_num,
        'x' | 'X' | '*' => first_num * second_num,
        _ => panic!("Invalid operator used"),
    }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}
