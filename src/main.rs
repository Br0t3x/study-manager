mod utils;

use std::io::stdin;
// use crate::utils::utils::*;
// use word_processing::*;

fn main() {
    let mut input = String::new();
    let inp = stdin();

    loop {
        match inp.read_line(&mut input) {
            Ok(_) => {
                read_input(input.trim());
            },
            Err(err) => {
                println!("Error when reading input: {err}");
            }
        }
    }
}

fn read_input(input: &str) {
    // let terms = get_terms(input);

}