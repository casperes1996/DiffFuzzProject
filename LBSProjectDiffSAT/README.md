
# How to run

This project directory contains a .devcontainer for Docker to ease setup. There is a setup script that auto runs inside the container after the container is up to get Rust ready.
Furthermore, for calling SAT solvers with .cnf files we wanted to be able to quickly read/write these many randomly generated files "to disk" without also potentially performing millions of tiny writes on disk per second while fuzzing.
Therefore there is a volumeSetup.sh script that sets up a Ram Disk (script is to be run on macOS. A Linux version can be adapted with mounting a tmpfs volume). It is expected this folder exists before running the fuzzer. If you do not want to run this in a RAM Disk, you can just make the folder as a regular mkdir directory.

After all of that you can simply use
cargo run main
to run the fuzzer on the SAT instances. 

To tear down the Ram Disk again (if created with the macOS volumeSetup.sh script) you can simply run the script again. It looks if the directory is there or not and changes mode from create to reclaim based on it
