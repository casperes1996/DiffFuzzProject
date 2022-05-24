# DiffFuzzProject
Language Based Security Project focusing on differential fuzzing

## Structure

This repository is divided into three directories, each acting as a self-contained sub-project.
These sub-projects includes everything necessary to apply differential fuzzing to primality testing, red black trees and SAT solvers, as described in the project report this repository is associated with.

Each project comes with a devcontainer for Docker that should make it easy to run all the targets. The SAT sub-project creates a large amount of files to use as input for the SAT solvers and one of the solvers also outputs its results as files. To run faster and ease wear on disks,
the SAT sub-project is set up to make use of a RAM disk. See the README file in the sub-project directory for details.

In all sub-projects, running `cargo fuzz run fuzz_target_1`will run differential fuzzing with libFuzzer as the input generating backend, and `cargo run main`will run it with our own input generation.
