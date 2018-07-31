#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    // this might fail in CI
    let mut app = build_cli();
    app.gen_completions("{{bin}}", Shell::Bash, "completions/");
    app.gen_completions("{{bin}}", Shell::Fish, "completions/");
    app.gen_completions("{{bin}}", Shell::Zsh, "completions/");
    app.gen_completions("{{bin}}", Shell::PowerShell, "completions/");
}
