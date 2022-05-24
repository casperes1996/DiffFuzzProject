#![allow(non_snake_case)]
use arbitrary::*;
use is_prime::*;
use LBSProjectDiffPrimality::miller_rabin;

fn main() -> Result<()> {
    let mut iteration = 0;
    loop {
        if iteration % 1000 == 0 {
            println!("Iteration number: {}", iteration);
        }
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

pub fn generate_random_buffer(size: usize) -> Vec<u8> {
    let mut rng = urandom::new();
    let mut buff = vec![0u8; size];
    rng.fill_bytes(&mut buff);
    //println!("{:?}", buff);
    return buff;
}
