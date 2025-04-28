use crate::paths::calc_nums::{calculate_numbers_addition, calculate_numbers_mult_diff, calculate_resolve_brackets};

use super::calculating::calculate_numbers_powers;

// Rechenregeln von Klammer, Potenzen, Punkt vor Strich und zu Addition und Subtraktion,
// von links nach rechts brauche ich nicht da mein Tool das sowieso
// macht.
pub fn rules_for_calculation(formula: Vec<String>){
    let mut operator_count: Vec<usize> = count_operators(&formula);
    let mut changed_formula: Vec<String> = formula;
    let mut last_calculate: String = String::new();

    if operator_count[0] > 0 {
        println!("Klammern berechnen und auflösen.\n");
        changed_formula = calculate_resolve_brackets(changed_formula, operator_count[0]);
        println!("--------------------------");
    }
    if operator_count[1] > 0 {
        println!("Potenzen werden berechnet.\n");
        changed_formula = calculate_numbers_powers(changed_formula);
        println!("--------------------------");

        if let Some(temp) = operator_count.get_mut(1) {*temp = 0;}
    }
    if operator_count[2] > 0 {
        println!("Berechnung (Punkt vor Strich)\n");
        changed_formula = calculate_numbers_mult_diff(changed_formula);
        println!("--------------------------");

        if let Some(temp) = operator_count.get_mut(2) {*temp = 0;}
    }
    if operator_count[3] > 0 {
        println!("Berechnung (Addition und Subtraktion)\n");
        last_calculate = calculate_numbers_addition(changed_formula);
        println!("--------------------------");
    }
      
    println!("Endergebnis {:.2}", last_calculate.parse::<f64>().expect("Invalid Number"));
    println!("Erfolg!");
}

// Hier werden alle Operatoren gezählt, wichtig um zu wissen
// wie oft eine Funktion zum berechnen aufgerufen werden muss 
// oder auch nicht. Und damit die Rechenregeln in der richtigen
// Reihenfolge ablaufen.
fn count_operators(numbers: &Vec<String>) -> Vec<usize> {
    let mut operator_count: Vec<usize> = Vec::new();
    
    let mut count_brackets: usize = 0;
    let mut count_powers: usize = 0;
    let mut count_mult_diff: usize = 0;

    for i in numbers.iter() {
        if i.contains("(") | i.contains(")") {count_brackets += 1;}
        if i.contains("^") {count_powers += 1;}
        if i.contains("*") | i.contains("/") {count_mult_diff += 1;}
    }
    
    let operator_total: usize = count_powers + count_mult_diff;

    operator_count.push(count_brackets);
    operator_count.push(count_powers);
    operator_count.push(count_mult_diff);
    operator_count.push(operator_total);

    println!();
    println!("Klammern           : {}", count_brackets);
    println!("Potenzen           : {}", count_powers);
    println!("Multi/Diff         : {}", count_mult_diff);
    println!("Gesamt             : {}", operator_total);
    println!("Vektorinhalt Count : {:?}", operator_count);

    return operator_count
}