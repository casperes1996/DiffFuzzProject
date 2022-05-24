#![no_main]
use libfuzzer_sys::fuzz_target;
use is_prime::*;
use LBSProjectDiffPrimality::miller_rabin;

fuzz_target!(|data: u64| {
    if data != 0 {
        let res_1_bool = is_prime(&data.to_string());
        let res_2_int = miller_rabin(data, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]); //using the default recommended bases
        let res_2_bool = if res_2_int == 0 { true } else { false };
        println!("{}", data);
        if res_1_bool != res_2_bool {
            println!("---------------------------");
            println!("Test result is: {}", data);
            println!("Program one said: {}", res_1_bool);    
            println!("Program two said: {}", res_2_bool);    
            println!("---------------------------");
        }
        assert_eq!(res_1_bool, res_2_bool);
    }   
});