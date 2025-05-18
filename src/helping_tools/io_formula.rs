use std::io::{self, Write};

pub fn input_formula() -> String {

    io::stdout().flush().expect("Ungültige Eingabe");

    let mut input_term: String = String::new();
    io::stdin().read_line(&mut input_term).expect("Ungültige Eingabe");
    
    return input_term.trim_end().to_string();
}