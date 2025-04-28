use super::display_terminal::display_terminals;

// Diese Funktion ist für andere sichtbar und macht die
// Stringmanipulationen verfügbar.
pub fn strings_refactor(crazy_string: String) -> Vec<String> {
    let result_string: String = remove_whitespaces(crazy_string);
    let result_validation = validation_brackets_operators(result_string);
    let terms_replaced: String = terms_replace_operators(result_validation);
    return split_terms(terms_replaced)
}

// Um überflüssige Leerzeichen zu entfernen
// Damit sind auch Strings in der Form " 3  +  5  * 3" möglich
fn remove_whitespaces(with_whitespaces: String) -> String {
    let result_string: String = with_whitespaces.chars().filter(|c| !c.is_whitespace()).collect();
    
    display_terminals("Leerzeichen entfernt".to_string(), &result_string);
    
    result_string
}

// Extra Funktionen für die doppelten Operatoren hintereinander
// und auf vollständigkeit der Klammern.
fn validation_brackets_operators(brackets_ops: String) -> String {
    let mut count_brackets: usize = 0;
    let brackets_ops_string: String = brackets_ops;

    let first_char: char = brackets_ops_string.chars().nth(0).unwrap();

    // Prüfen ob das erste Zeichen des Strings korrekt
    // anfängt. Bei der Berechnung dürfen nur +, - oder eine Zahl sein
    // den Anfang machen. TODO: später für Variablen
    if first_char == '-' || first_char == '+' || first_char.is_digit(10) {
        for terms in brackets_ops_string.chars() {
            if terms == ')' && count_brackets == 0 {
                panic!("Darf nicht mit einer ')' Klammer beginnen")
            }
            if terms == '(' || terms == ')' {
                count_brackets += 1;
            }
        }
        // prüfen ob die Klammern vollständig sind
        if count_brackets % 2 != 0 {
            panic!("Klammern nicht vollständig!")
        }
    }
    else {
        panic!("Anfang der Formel muss mit +,- oder einer Zahl beginnen");
    }
    return brackets_ops_string
    
}

// Mit dieser Funktion werden nur Leerzeichen vor dem Operator gesetzt, um
// sie in der Funktion "split_terms" besser zu teilen.
fn terms_replace_operators(splitted_equation: String) -> String {
    let mut terms_replaced: String = String::new();

    for terms in splitted_equation.chars() {
            match terms {
                '(' => terms_replaced.push_str("( "),
                ')' => terms_replaced.push_str(" )"),
                '+' => terms_replaced.push_str( " +"),
                '-' => terms_replaced.push_str(" -"),
                '*' => terms_replaced.push_str(" * "),
                '/' => terms_replaced.push_str(" / "),
                '^' => terms_replaced.push_str(" ^ "),
                _ => terms_replaced.push(terms),
        }
    }
    display_terminals("Leerzeichen vor den Operatoren".to_string(), &terms_replaced);
    terms_replaced
}

// Hier werden die einzelnen Terme nochmals gesplitted, damit man besser
// mit Ihnen rechnen kann.
fn split_terms(splitting_terms: String) -> Vec<String> {
    let mut splitted_terms: Vec<String> = splitting_terms.split(' ').map(str::to_string).collect();
    if splitted_terms[0] == "" {
        splitted_terms.remove(0);
    }
    display_terminals("Terme einzeln aufgeteilt".to_string(), &splitted_terms.join(" "));
    splitted_terms
}