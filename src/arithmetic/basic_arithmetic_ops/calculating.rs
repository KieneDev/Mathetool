use core::f64;
use std::ops::Index;
use crate::helping_tools::display_terminal::display_terminals_validate;

// Rechnregel Klammern auflösen und berechnen
pub fn calculate_resolve_brackets(numbers: Vec<String>, brackets_max_count: usize) -> Vec<String> {
    
    let result_brackets_vector: Vec<String> = numbers;

    println!("Vektorinhalt: {:?}", result_brackets_vector);
    println!("Anzahl Klammern: {}", brackets_max_count);

    let brackets_index_array: Vec<usize> = get_brackets_index(&result_brackets_vector);
    println!("Brackets Index {:?}", brackets_index_array);
    let mut result_splitted_brackets: Vec<Vec<String>> = extract_brackets(result_brackets_vector, &brackets_index_array);
    calculate_brackets(result_splitted_brackets.remove(1));
    println!("Splitted Vektor middle: {:?}", result_splitted_brackets[1]);
    println!();
    
    //println!("Index(1) = {:?}, Index(2) = {:?}", brackets_index_array[0], brackets_index_array[1]);
    todo!("\nFormel (Klammern) ist noch nicht fertig\n");
    //return Vec::new();
}

// Berechnung für die Klammern um sie danach aufzulösen
fn calculate_brackets(numbers: Vec<String>) {
    
}


// Rechenregel Potenzen
pub fn calculate_numbers_powers(numbers: Vec<String>) -> Vec<String> {
    let mut result_powers_vector: Vec<String> = numbers;
    let mut index: usize = 0;
    let mut counter_powers: usize = 0;

    println!("\nBerechnung Potenzen\n");

    while index < result_powers_vector.len(){

        // Potenzen
        if result_powers_vector[index].contains("^") {
            
            counter_powers += 1;

            println!("Vektor davor: {:?}", result_powers_vector);
            let result_powers: String = calculate_powers(&result_powers_vector, counter_powers, '^');
            
            removing_from_vector(&mut result_powers_vector, index, result_powers);
            println!("Vektor danach: {:?}", result_powers_vector);
            
            index = 0;
        }
        else {
            index += 1;
        }
    }
    return result_powers_vector
}

// Funktion für die Potenzen
fn calculate_powers(numbers: &Vec<String>, counter: usize, which_operator: char) -> String {
    let mut result_powers: String = String::new();
    let mut operation: String = String::new();
    let result: f64;
    let mut left_right: Vec<f64> = Vec::new();
    
    for (i, num) in numbers.iter().enumerate() {
        if num.contains(which_operator) {

            left_right = left_right_terms(&numbers[i - 1], &numbers[i + 1]).to_vec();
            
            match which_operator {
                '^' => {
                    result = f64::powf(left_right[0], left_right[1]);
                    operation = "Potenz".to_string();
                }
                _ => return "Invalid Number".to_string()
            }
            result_powers = result.to_string();  
            break;
        }
    }
    print!("{}. ", counter);
    display_terminals_validate(operation, &left_right[0].to_string(), &left_right[1].to_string(), &which_operator.to_string(), &result_powers);

    return result_powers
}

// Hier wird die vorletzte Rechenregel angewendet die Punkt vor Strich
// Berechnung.
pub fn calculate_numbers_mult_diff(numbers: Vec<String>) -> Vec<String> {
    
    let mut result_mul_div_vector: Vec<String> = numbers;
    let mut index: usize = 0;
    let mut counter_mult: usize = 0; 
    let mut counter_div: usize = 0;

    while index < result_mul_div_vector.len(){

        // Multiplikation
        if result_mul_div_vector[index].contains("*") {
            
            counter_mult += 1;

            println!("Vektor davor: {:?}", result_mul_div_vector);
            let result_mult: String = calculate_mult_diff(&result_mul_div_vector, counter_mult, '*');
            
            removing_from_vector(&mut result_mul_div_vector, index, result_mult);
            println!("Vektor danach: {:?}", result_mul_div_vector);
            
            index = 0;
        }

        // Division
        else if result_mul_div_vector[index].contains("/") {

            counter_div += 1;

            println!("Vektor davor: {:?}", result_mul_div_vector);
            let result_mult: String = calculate_mult_diff(&result_mul_div_vector, counter_div, '/');
            
            removing_from_vector(&mut result_mul_div_vector, index, result_mult);
            println!("Vektor danach: {:?}", result_mul_div_vector);

            index = 0;
        }
        else {
            index += 1;
        }
    }

    return result_mul_div_vector
}

// Funktion für die Multiplikation und die Division
fn calculate_mult_diff(numbers: &Vec<String>, counter: usize, which_operator: char) -> String {
    let mut result_mult: String = String::new();
    let result: f64;
    let mut operation: String = String::new();
    let mut left_right: Vec<f64> = Vec::new();

    for (i, num) in numbers.iter().enumerate() {
        if num.contains(which_operator) {
            left_right = left_right_terms(&numbers[i - 1], &numbers[i + 1]).to_vec();
            
            match which_operator {
                '*' => {
                    result = left_right[0] * left_right[1];
                    operation = "Multiplikation".to_string();
                }
                '/' => {
                    result = left_right[0] / left_right[1];
                    operation = "Division".to_string();
                }
                _ => return "Invalid Number".to_string()
            }

            result_mult = result.to_string();  
            break;
        }
    }
    print!("{}. ", counter);
    display_terminals_validate(operation, &left_right[0].to_string(), &left_right[1].to_string(), &which_operator.to_string(), &result_mult);

    return result_mult 
}

// Mit dieser Funktion werden die Zahlen sowohl positiv
// und negativ miteinander addiert. Das ist der letzte Schritt
// bei den Rechenregeln. Da hier das Vorzeichen mitgenommen wird
// muss man hier nicht auf die Vertauschungsregel achten.
pub fn calculate_numbers_addition(numbers: Vec<String>) -> String {
    
    let mut result: f64 = 0.0;

    for num in numbers.iter() {
        match num.parse::<f64>() {
            Ok(number) => result += number,
            Err(_) => return "Error: invalid number".to_string(),
        }
    }
    return result.to_string()
}


// Kleines Refactoring weil das entfernen und hinzufügen mehrmals
// vorkommt, habe ich sie ausgelagert.
fn removing_from_vector(numbers_vector: &mut Vec<String>, index: usize, result: String) {
    // Das aktuelle Element wird ersetzt
    numbers_vector[index] = result;
    // Das Element danach wird entfernt
    numbers_vector.remove(index + 1);
    // Das Element danvor wird entfernt
    numbers_vector.remove(index - 1);
}

// Umwandlung des linken und rechten Operanden, da es öfters 
// benutzt wird habe ich sie ausgelagert
fn left_right_terms(left: &str, right: &str) ->[f64;2] {
    [
        left.parse::<f64>().expect("Invalid Number"),    
        right.parse::<f64>().expect("Invalid Number")
    ]
}

// Funktionen um die Klammern aus der Formel mittels Index
// zu extrahieren
fn extract_brackets(numbers_brackets: Vec<String>, brackets_index: &Vec<usize>) -> Vec<Vec<String>> {

    let split_brackets_vector_right: (&[String], &[String]) = numbers_brackets.split_at(brackets_index[3] + 1);
    let split_vector_brackets: Vec<String> = split_brackets_vector_right.0.to_vec();
    let split_brackets_vector_left: (&[String], &[String]) = split_vector_brackets.split_at(brackets_index[0]);

    let mut result_splitting: Vec<Vec<String>> = Vec::new();

    result_splitting.push(split_brackets_vector_left.0.to_vec());
    result_splitting.push(split_brackets_vector_left.1.to_vec());
    result_splitting.push(split_brackets_vector_right.1.to_vec());

    return result_splitting;
}

// Funktion zum herausfinden wo die erste und die letzte Klammer ist
// um später direkt darauf zuzugreifen ohne den String ein weiteres
// mal von vorne zu iterieren.
fn get_brackets_index(numbers: &Vec<String>) -> Vec<usize> {

    let mut brackets_index_vector: Vec<usize> = Vec::new();

    for (index, num) in &mut numbers.iter().enumerate() {
        if num.contains("(") || num.contains(")") {
            brackets_index_vector.push(index); 
        }
    }
    return brackets_index_vector
}
