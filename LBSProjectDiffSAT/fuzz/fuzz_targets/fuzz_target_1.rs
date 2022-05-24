#![no_main]
#![allow(non_snake_case)]
use libfuzzer_sys::fuzz_target;
use LBSProjectDiffSAT::*;
use std::process::{Command,Stdio};
use execute::Execute;
#[macro_use] extern crate guard;

fuzz_target!(|dimac_input: Module| {
    unsafe {
        static mut I: i32 = 0; // current input file
        println!("{}",dimac_input.output);
        generate_cnf_file_module(dimac_input, I);
        
        let pathToInput = format!("/workspaces/LanguageBasedSecurity/LBSProjectDiffSAT/FuzzVolume/inputFile{}.cnf", I);
        
        let mut splr = Command::new(&"splr");
        splr.arg("-o");
        splr.arg("/workspaces/LanguageBasedSecurity/LBSProjectDiffSAT/FuzzVolume");
        splr.arg(&pathToInput);
        guard_unwrap!(let Some(splr_exit_code) = splr.execute().unwrap());
        let splr_sat = if splr_exit_code == 20 {SatStatus::UNSAT} else if splr_exit_code == 10 {SatStatus::SAT} else {SatStatus::UNDETERMINED};
        
        
        // Run screwsat - A new use scope is set since it will shadow the "Solver" struct identity from creation of SPLR
        let mut screwsat = Command::new(&"screwsat");
        screwsat.arg(&pathToInput);
        
        screwsat.stdout(Stdio::piped());
        let screwsat_out = screwsat.execute_output().unwrap();
        if screwsat_out.status.code() != Some(0) {
            eprintln!("Screwsat neither sat n'or unsat!");
        }
        
        let screwsat_string = String::from_utf8(screwsat_out.stdout).unwrap();
        
        let screwsat_sat = if screwsat_string.contains("UNSAT") {SatStatus::UNSAT} else if screwsat_string.contains("INDET") {SatStatus::UNDETERMINED} else {SatStatus::SAT};
        
        println!("\n{:?}, {:?}\n", screwsat_sat, splr_sat);
        
        if screwsat_sat != splr_sat {
            println!("Found divergence! SPLR: {:?}, Screwsat: {:?}", splr_sat, screwsat_sat);
            assert!(false);
        }
        
        // Increment to go to next file - comment out if you want to always only worok on one cnf file and not keep corpus of passed tests.
        I += 1;
    }
});
