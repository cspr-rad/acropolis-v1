mod prover;
use std::path::PathBuf;

use acropolis::{run, Cli, Command};
use clap::Parser;
fn main() {
    let cli = Cli::parse();
    run(cli);
}
