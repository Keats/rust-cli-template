#[macro_use]
extern crate clap;

mod cli;
mod errors;

fn main() {
    let matches = cli::build_cli().get_matches();
    // Do your thing here
}
