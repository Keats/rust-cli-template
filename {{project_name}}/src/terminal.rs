use std::io::prelude::*;

use term;

// Some examples on how to use the term crate

pub fn error(message: &str) {
    if let Some(mut t) = term::stderr() {
        match t.fg(term::color::BRIGHT_RED) {
            Ok(_) => {
                write!(t, "{}", message).unwrap();
                t.reset().unwrap();
            },
            Err(_) => writeln!(t, "{}", message).unwrap()
        };
    } else {
        eprint!("{}", message);
    }
}


pub fn success(message: &str) {
    if let Some(mut t) = term::stdout() {
        match t.fg(term::color::GREEN) {
            Ok(_) => {
                write!(t, "{}", message).unwrap();
                t.reset().unwrap();
            },
            Err(_) => writeln!(t, "{}", message).unwrap()
        };
    } else {
        eprint!("{}", message);
    }
}

