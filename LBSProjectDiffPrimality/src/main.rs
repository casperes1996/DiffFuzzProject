#![allow(non_snake_case)]
use is_prime::*;
use arbitrary::*;

fn main() -> Result<()> {
    let mut iteration = 0;
    loop {
        if iteration %1000 == 0 {println!("Iteration number: {}", iteration);}
        let buffer = generate_random_buffer(8);
        let mut u = Unstructured::new(&buffer);

        let data = u64::arbitrary(&mut u)?;
        let res_1_bool = is_prime(&data.to_string());
        let res_2_int = miller_rabin(data, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]); //using the default recommended bases
        let res_2_bool = if res_2_int == 0 { true } else { false };
        if res_1_bool != res_2_bool {
            println!("---------------------------");
            println!("Test result is: {}", data);
            println!("Program one said: {}", res_1_bool);    
            println!("Program two said: {}", res_2_bool);    
            println!("---------------------------");
        }
        assert_eq!(res_1_bool, res_2_bool);
        iteration += 1;
    }
}

fn modulo_power(mut base: u64, mut power: u64, modulo: u64) -> u64 {
    base %= modulo;
    if base == 0 {
        return 0; // return zero if base is divisible by modulo
    }
    let mut ans: u128 = 1;
    let mut bbase: u128 = base as u128;
    while power > 0 {
        if (power % 2) == 1 {
            ans = (ans * bbase) % (modulo as u128);
        }
        bbase = (bbase * bbase) % (modulo as u128);
        power /= 2;
    }
    ans as u64
}

fn check_prime_base(number: u64, base: u64, two_power: u64, odd_power: u64) -> bool {
    // returns false if base is a witness
    let mut x: u128 = modulo_power(base, odd_power, number) as u128;
    let bnumber: u128 = number as u128;
    if x == 1 || x == (bnumber - 1) {
        return true;
    }
    for _ in 1..two_power {
        x = (x * x) % bnumber;
        if x == (bnumber - 1) {
            return true;
        }
    }
    false
}

pub fn miller_rabin(number: u64, bases: &[u64]) -> u64 {
    // returns zero on a probable prime, and a witness if number is not prime
    // A base set for deterministic performance on 64 bit numbers is:
    // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    // another one for 32 bits:
    // [2, 3, 5, 7], with smallest number to fail 3'215'031'751 = 151 * 751 * 28351
    // note that all bases should be prime
    if number <= 4 {
        match number {
            2 => return 0,
            3 => return 0,
            _ => return number,
        }
    }
    if bases.contains(&number) {
        return 0;
    }
    let two_power: u64 = (number - 1).trailing_zeros() as u64;
    let odd_power = (number - 1) >> two_power;
    for base in bases {
        if !check_prime_base(number, *base, two_power, odd_power) {
            return *base;
        }
    }
    0
}


pub fn generate_random_buffer(size: usize) -> Vec<u8> {
    let mut rng = urandom::new();
    let mut buff = vec![0u8;size];
    rng.fill_bytes(&mut buff);
    //println!("{:?}", buff);
    return buff;
}
