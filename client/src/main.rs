mod prover;

use acropolis::{run, Cli};
use clap::Parser;
fn main() {
    let cli = Cli::parse();
    run(cli);
}
