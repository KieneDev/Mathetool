use core::f64;
use crate::helping_tools::display_terminal::display_terminals_validate;

// Rechenregel Potenzen
pub fn calculate_numbers_powers(numbers: Vec<String>) -> Vec<String> {
    let mut result_powers_vector: Vec<String> = numbers.to_vec();
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

// Funktion für die Multiplikation und die Division
fn calculate_mult_diff(numbers: &Vec<String>, counter: usize, which_operator: char) -> String {
    let mut result_mult: String = String::new();
    let result: f64;
    let mut operation: String = String::new();
    let mut left_right: Vec<f64> = Vec::new();
    let mut counter_mult: usize = 0;

    for (i, num) in numbers.iter().enumerate() {
        if num.contains(which_operator) {
            left_right = left_right_terms(&numbers[i - 1], &numbers[i + 1]).to_vec();
            
            match which_operator {
                '*' => {
                    counter_mult += 1;
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