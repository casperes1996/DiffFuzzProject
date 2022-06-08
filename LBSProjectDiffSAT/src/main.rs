#![allow(non_snake_case)]
use arbitrary::Unstructured;
use execute::Execute;
use std::process::{Command, Stdio};
use LBSProjectDiffSAT::*;
#[macro_use]
extern crate guard;

pub fn main() -> std::io::Result<()> {
    let mut i = 0;
    loop {
        println!("Input number: {}", i);
        let buffer = generate_random_buffer(4002);
        let u = Unstructured::new(&buffer); // 4002 should be enough bytes for even the biggest cnf file as currently scaled.
        generate_cnf_file(u, i); // write a CNF file to the RAM Disk based on u with file name ending in number i - Ram Disk must exist

        // We have generated the cnf
        let pathToInput = format!(
            "/workspaces/DiffFuzzProject/LBSProjectDiffSAT/FuzzVolume/inputFile{}.cnf",
            i
        );
        // Run SPLR
        let mut splr = Command::new(&"splr");
        splr.arg("-o");
        splr.arg("/workspaces/LanguageBasedSecurity/LBSProjectDiffSAT/FuzzVolume");
        splr.arg(&pathToInput);
        guard_unwrap!(let Some(splr_exit_code) = splr.execute().unwrap());
        let splr_sat = if splr_exit_code == 20 {
            SatStatus::UNSAT
        } else if splr_exit_code == 10 {
            SatStatus::SAT
        } else {
            SatStatus::UNDETERMINED
        };

        // Run screwsat - A new use scope is set since it will shadow the "Solver" struct identity from creation of SPLR
        let mut screwsat = Command::new(&"screwsat");
        screwsat.arg(&pathToInput);

        screwsat.stdout(Stdio::piped());
        let screwsat_out = screwsat.execute_output().unwrap();
        if screwsat_out.status.code() != Some(0) {
            eprintln!("Screwsat neither sat n'or unsat!");
        }

        let screwsat_string = String::from_utf8(screwsat_out.stdout).unwrap();

        let screwsat_sat = if screwsat_string.contains("UNSAT") {
            SatStatus::UNSAT
        } else if screwsat_string.contains("INDET") {
            SatStatus::UNDETERMINED
        } else {
            SatStatus::SAT
        };

        println!("\n{:?}, {:?}\n", screwsat_sat, splr_sat);

        if screwsat_sat != splr_sat {
            println!(
                "Found divergence! SPLR: {:?}, Screwsat: {:?}",
                splr_sat, screwsat_sat
            );
            assert!(false);
        }

        // Increment to go to next file - comment out if you want to always only worok on one cnf file and not keep corpus of passed tests.
        i += 1;
    }
}

pub fn generate_random_buffer(size: usize) -> Vec<u8> {
    let mut rng = urandom::new();
    let mut buff = vec![0u8; size];
    //println!("{:?}", buff);
    rng.fill_bytes(&mut buff);
    //println!("{:?}", buff);
    return buff;
}
