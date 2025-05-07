#![allow(dead_code)]

use arithmetic::basic_arithmetic_ops::calculation_rules::rules_for_calculation;
use helping_tools::display_terminal::display_terminals;

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;

fn main() {
    //let equation_string: String = "(100+2)^2-2*(5*(8/(9-12)+5)/2)*8+1*(2+5)*9".to_string();
    // let equation_string: String = "100+2*2-2^5*8/9-12+5/2*8+12".to_string();
    let equation_string: String = "(9-12)".to_string();
    // let equation_string: String = "3 + (-2) * (4 - 1)^2 - (-6 / 2 + 3)".to_string();
    println!();
    display_terminals("Original Formel".to_string(), &equation_string);

    let splitted_terms: Vec<String> = paths::str_manipulation::strings_refactor(equation_string);
    
    println!("Ergebnis: {:?}", rules_for_calculation(splitted_terms));
}