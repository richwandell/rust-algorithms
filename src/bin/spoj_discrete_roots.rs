use std::io::{self};
use std::process;

fn powmod(mut a: i32, mut b: i32, p: i32) -> i32 {
    let mut res = 1;
    while b > 0 {
        if b & 1 {
            res = res * a % p;
        }
        a = a * a % p;
        b >>= 1;
    }
    return res;
}

fn generator(p: i32) -> i32 {
    let mut fact: Vec<i32> = Vec::new();
    let phi: i32 = p-1;
    let mut n: i32 = phi;
    let mut i: i32 = 2;

    while i * i <= n {
        if n % i == 0 {
            fact.push(i);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 1;
    }

    if n > 1 {
        fact.push(n);
    }

    let mut res = 2;
    while res <= p {
        let mut ok = true;
        for factor in fact {
            if powmod(res, phi / factor, p) == 1 {
                ok = false;
                break
            }
        }
        if ok {
            return res
        }
        res+=1;
    }

    return -1;
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        let split: Vec<&str> = input.split(" ").collect();
        if split.len() < 3 {
            break
        }

        let n: i32 = split[0].trim().parse().unwrap();
        let k: i32 = split[1].trim().parse().unwrap();
        let a: i32 = split[2].trim().parse().unwrap();
    }
}