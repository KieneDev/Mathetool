use core::f64;
use crate::helping_tools::display_terminal::display_terminals_validate;
use num_complex::Complex;

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
                    let base = left_right[0];
                    let exponent = left_right[1];
                    
                    let complex_result: Complex<f64> = Complex::new(base, 0.0).powf(exponent);

                    result = complex_result.re;
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

pub fn calculate_numbers_addition(numbers: Vec<String>) -> Vec<String> {
    
    let mut result_add_sub_vector: Vec<String> = numbers;
    let mut index: usize = 0;
    let mut counter_mult: usize = 0; 
    let mut counter_div: usize = 0;

    while index < result_add_sub_vector.len(){

        // Multiplikation
        if result_add_sub_vector[index].contains("+") {
            
            counter_mult += 1;

            println!("Vektor davor: {:?}", result_add_sub_vector);
            let result_add: String = calculate_addition(&result_add_sub_vector, counter_mult, '+');
            
            removing_from_vector(&mut result_add_sub_vector, index, result_add);
            println!("Vektor danach: {:?}", result_add_sub_vector);
            
            index = 0;
        }

        // Division
        else if result_add_sub_vector[index].contains("-") {

            counter_div += 1;

            println!("Vektor davor: {:?}", result_add_sub_vector);
            let result_add: String = calculate_addition(&result_add_sub_vector, counter_div, '-');
            
            removing_from_vector(&mut result_add_sub_vector, index, result_add);
            println!("Vektor danach: {:?}", result_add_sub_vector);

            index = 0;
        }
        else {
            index += 1;
        }
    }

    return result_add_sub_vector
}

fn calculate_addition(numbers: &Vec<String>, counter: usize, which_operator: char) -> String {
    
    let mut result_add: String = String::new();
    let result: f64;
    let mut operation: String = String::new();
    let mut left_right: Vec<f64> = Vec::new();

    for (i, num) in numbers.iter().enumerate() {
        if num.contains(which_operator) {
            left_right = left_right_terms(&numbers[i - 1], &numbers[i + 1]).to_vec();
            
            match which_operator {
                '+' => {
                    result = left_right[0] + left_right[1];
                    operation = "Addition".to_string();
                }
                '-' => {
                    result = left_right[0] - left_right[1];
                    operation = "Subtraktion".to_string();
                }
                _ => return "Invalid Number".to_string()
            }

            result_add = result.to_string();  
            break;
        }
    }
    print!("{}. ", counter);
    display_terminals_validate(operation, &left_right[0].to_string(), &left_right[1].to_string(), &which_operator.to_string(), &result_add);
    
    return result_add.to_string()
}


// Kleines Refactoring weil das entfernen und hinzufügen mehrmals
// vorkommt, habe ich sie ausgelagert.
pub fn removing_from_vector(numbers_vector: &mut Vec<String>, index: usize, result: String) {
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


