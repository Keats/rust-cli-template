#[macro_use]
extern crate clap;
{%- if fancy_term %}

extern crate term;
{%- endif %}

mod cli;
mod errors;
{%- if fancy_term %}
mod terminal;
{%- endif %}

fn main() {
    let matches = cli::build_cli().get_matches();
    // Do your thing here
}
