use crate::helping_tools::io_formula::input_formula;
use crate::helping_tools::display_terminal::clearscreen;
use crate::arithmetic::basic_arithmetic_ops::calculation_rules::rules_for_calculation;
use crate::helping_tools::display_terminal::display_terminals;
use crate::paths;

pub fn mathtool_menue_terminal() {
    
    loop {
        clearscreen();

        main_print_menue();

        let mut input: String = input_formula();

        match input.as_str().trim() {
            "y" | "Y" => { break; }
            "1" => {
                sub_menue_arithmetic();
            }
            _ => { 
                print!("Ungültige Eingabe");
            }
        }
        input = input_formula();
    }
}

fn main_print_menue() {
    println!("Mathetool by Super(d/g)oof\n");
    println!("(1). Arithmetik\n");
    println!("Eingabe als Nummer, zum beenden (y/Y) eingeben.\n");
    print!("Ihre Eingabe: ");
}
fn sub_menue_arithmetic() {
    let mut input: String = String::new();

    loop {
        clearscreen();
        
        print!("Ihr Term: ");
        input = input_formula();

        if input.trim() == "" { continue; }

        display_terminals("Original Formel".to_string(), &input);

        let splitted_terms: Vec<String> = paths::str_manipulation::strings_refactor(input);
        
        println!("Ergebnis: {:?}", rules_for_calculation(splitted_terms));

        println!("Weiter (w) oder zurück (b)");
        print!("Ihre Eingabe: ");

        input = input_formula();

        match input.as_str() {
            "b" | "B" => { break; }
            "w" | "W" => { continue; }
            _ => { print!("Ungültige Eingabe") }
        }
    }
}