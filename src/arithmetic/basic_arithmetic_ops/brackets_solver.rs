
use crate::arithmetic::basic_arithmetic_ops::calculation_rules::rules_for_calculation;
use super::index_operators::find_index;

// Rechnregel Klammern auflösen und berechnen
pub fn calculate_resolve_brackets(numbers: Vec<String>) -> Vec<String> {
    
    let mut calculate_index: usize = 0;
    let mut result_brackets_calc: Vec<String> = numbers;

    while calculate_index < result_brackets_calc.len() {
        let mut highest_index_start: usize = 0;
        let mut highest_index_brackets: (usize, usize) = (0, 0);

        // Da ich den Index-Vektor verkleinere muss ich die Indizes
        // neu berechnen.
        let mut result_index_brackets: Vec<(usize, usize)> = find_index(&result_brackets_calc);

        // Hier hole ich den höchsten Index für die Klammerpaarungen
        // für die Auflösungen nach dem Prinzip von innen nach aussen, und 
        // Klammernpaare die unabhängig von den verschachtelten sind. Mit
        // dieser Art wird die Reihenfolge eingehalten.
        for i in result_index_brackets.iter() {
            println!("Vektor Klammer: [{:?}, {:?}]", i.0, i.1);
            if i.0 >= highest_index_start {
                highest_index_start = i.0;
                highest_index_brackets.0 = i.0;
                highest_index_brackets.1 = i.1;
            }
        }

        if result_index_brackets.len() > 0 {

            // Funktion zum berechnen und zum entfernen der Klammern
            calculate_brackets(&mut result_brackets_calc, highest_index_brackets);

            let mut index: usize = 0;

            // Hier wird das Klammerpaar aus der Vektor-Liste entfernt
            while index < result_index_brackets.len() {
                if result_index_brackets[index] == (highest_index_brackets.0, highest_index_brackets.1) {
                    result_index_brackets.remove(index);
                }
                index += 1;
            }

            calculate_index = 0;
        }
        else {
            calculate_index += 1;
        }
    }
    return result_brackets_calc
}


// Berechnung für die Klammern um sie danach aufzulösen, und um
// die Formel zu bereinigen für die nächsten Schritte.
fn calculate_brackets(numbers: &mut Vec<String>, brackets_index: (usize, usize)) {

    println!("Vektor davor: {:?}", numbers);

    // Ich benutze hier drain, damit kann ich den Teil für die Berechnung
    // heraus entfernen kann. Dadurch wird mir der Teil gegeben den ich berechnen kann.
    let mut removed_slices: Vec<String> = numbers.drain(brackets_index.0..=brackets_index.1).collect();

    // Die Klammern müssen entfernt werden sonst würde es zu
    // einer Endlosschleife kommen, weil die Funktion rules_for_calculation 
    // sie immer mit einbezieht.
    removed_slices.remove(0);
    removed_slices.remove(removed_slices.len()-1);

    println!("Herausgenommener Term: {:?}", removed_slices);

    // Berechnen nach den Regeln/Reihenfolge der Mathematik
    let bracket_result: String = rules_for_calculation(removed_slices);

    // Zuletzt wird hier das Ergebnis in den Verktor wieder eingefügt,
    // was vorher entnommen wurde.
    numbers.insert(brackets_index.0, bracket_result);

    println!("Vektor danach: {:?}", numbers);
}