#![allow(dead_code)]

use arithmetic::basic_arithmetic_ops::{calculation_rules::{rules_for_calculation, OperatorInfo}, index_operators::find_index, operators::operator};
use helping_tools::display_terminal::display_terminals;

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;

fn main() {
    let equation_string: String = "(100+2)*2-2^(5*(8/(9-12)+5)/2)*(8+12)".to_string();
    println!();
    display_terminals("Original Formel".to_string(), &equation_string);

    let splitted_terms: Vec<String> = paths::str_manipulation::strings_refactor(equation_string);

    println!("Vektor nach der Bereinigung: {:?}", splitted_terms);

    let result_index_brackets: Vec<(usize, usize)> = find_index(&splitted_terms);

    println!("Index Klammern: {:?}", result_index_brackets);

    println!("Get_operator: {:?}", operator::pow_operator());

    //todo!("bitte weiter machen!");

    let found_ops: Vec<OperatorInfo> = rules_for_calculation(splitted_terms);

    // let result_powers: Vec<String> = paths::calc_nums::calculate_numbers_powers(splitted_terms);

    // let result_mult_div: Vec<String> = paths::calc_nums::calculate_numbers_mult_diff(result_powers);

    // println!();
    // display_terminals("Addition mit Subtraktion".to_string(), &result_mult_div.join(" "));

    // let result_addition_all: String = paths::calc_nums::calculate_numbers_addition(result_mult_div);

    // let end_result: f64 = match result_addition_all.parse::<f64>() {
    //     Ok(result) => result,
    //     Err(_) => 0.0
    // };

    // display_terminals("Endergebnis".to_string(), &end_result.to_string());
    // println!();
}