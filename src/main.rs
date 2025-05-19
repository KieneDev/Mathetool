#![allow(dead_code)]

use menue::mathtool_menue_terminal;
use crate::arithmetic::basic_arithmetic_ops::calculation_rules::rules_for_calculation;

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;
mod menue;

fn main() {
    //let equation_string: String = "(100+2)^2-2*(5*(8/(9-12)+5)/2)*8+1*(2+5)*9".to_string();
    // let equation_string: String = "100+2*2-2^5*8/9-12+5/2*8+12".to_string();
    //let equation_string: String = "(3+2)^2 * (4 ^ (-3)) - 5 * (10 / (2 + 3)) + 8 ^ 2".to_string();
    //let equation_string: String = "-(5-5)".to_string();
    let equation_string: String = "4(6+5*10(5(2^2))123+6(5-4))".to_string();

    //mathtool_menue_terminal();

    let splitted_terms: Vec<String> = paths::str_manipulation::strings_refactor(equation_string);
        
    println!("Ergebnis: {:?}", rules_for_calculation(splitted_terms));
}