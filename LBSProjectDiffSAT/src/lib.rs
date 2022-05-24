#![allow(non_snake_case)]

use arbitrary::{Arbitrary, Result, Unstructured};
use std::fs;

#[derive(Debug)]
pub struct Module {
    pub output: String,
}

impl Arbitrary<'_> for Module {
    fn arbitrary(u: &mut Unstructured) -> Result<Self> {
        let mut temp_output = vec![];
        let number_of_var = u.int_in_range::<u8>(0..=255)? % 100 + 1; // Should be range 1..=100 but without limiting which bytes we can take from the stream
        let number_of_clauses = u.int_in_range::<u8>(0..=255)? % 100 + 1;

        temp_output.push(format!("p cnf {} {}\n", number_of_var, number_of_clauses));

        let mut number_of_var_in_clause;
        for _ in 1..=number_of_clauses {
            number_of_var_in_clause = u.int_in_range::<u8>(0..=255)? % 20 + 1; // Up to 20 variables in one clause without limiting which bytes we can consume
            for _ in 1..=number_of_var_in_clause {
                let extracted_var = u.int_in_range::<u8>(0..=255)? % number_of_var + 1;
                //deciding if var should be negated or not
                match u.int_in_range::<u8>(0..=255)? {
                    0u8..=127u8 => {
                        temp_output.push(format!("{} ", extracted_var));
                    }
                    128u8..=255u8 => {
                        temp_output.push(format!("-{} ", extracted_var));
                    }
                }
            }
            temp_output.push("0\n".to_string());
        }
        return Ok(Module {
            output: temp_output.concat(),
        });
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SatStatus {
    SAT,
    UNSAT,
    UNDETERMINED,
}

pub fn generate_cnf_file(mut u: Unstructured, i: i32) {
    if let Ok(out) = Module::arbitrary(&mut u) {
        // Only works after setting up the fuzz-volume RAM disk. Run volumeSetup script to make ram disk (macOS) - Alternatively a regular folder works, but is slower and will write many small files to disk.
        let _ = fs::write(format!("./FuzzVolume/inputFile{}.cnf", i), out.output);
    }
}

pub fn generate_cnf_file_module(m: Module, i: i32) {
    // Only works after setting up the fuzz-volume RAM disk. Run volumeSetup script to make ram disk (macOS) - Alternatively a regular folder works, but is slower and will write many small files to disk.
    let _ = fs::write(format!("./FuzzVolume/inputFile{}.cnf", i), m.output);
}
