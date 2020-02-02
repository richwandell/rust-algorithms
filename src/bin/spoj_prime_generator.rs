use std::io::{self};
use std::process;

fn is_prime(num: i32) -> bool {
    if num <= 3 {
        return num > 1
    } else if num % 2 == 0 || num % 3 == 0 {
        return false
    }

    let mut i: i32 = 5;

    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }

    return true
}

fn main() {

    let mut t: Option<i32> = None;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        match t {
            Some(_x) => {
                let split: Vec<&str> = input.split(" ").collect();
                if split.len() < 2 {
                    break
                }
                let m: i32 = split[0].trim().parse().unwrap();
                let n: i32 = split[1].trim().parse().unwrap();
                for i in m..n+1 {
                    let p: bool = is_prime(i);
                    if p {
                        println!("{}", i);
                    }
                }
                println!();
            },
            None => {
                let i: i32 = match input.trim().parse::<i32>() {
                    Ok(number) => number,
                    Err(_error) => process::exit(0)
                };
                t = Some(i);
            }
        }
    }
}
