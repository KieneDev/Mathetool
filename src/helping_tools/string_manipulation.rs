use super::display_terminal::display_terminals;

// Diese Funktion ist für andere sichtbar und macht die
// Stringmanipulationen verfügbar. Hier wird auch für die 
// Klammerberechnung noch ein bool hinzugefügt, was später
// abgefragt werden kann ob die Klammerberechnung nötig ist.
pub fn strings_refactor(crazy_string: String) -> Vec<String> {
    let result_string: String = remove_whitespaces(crazy_string);
    let terms_replaced: String = terms_replace_operators(result_string);
    let terms_splitted: Vec<String> = split_terms(terms_replaced);

    let negative_numbers: Vec<String> = validate_negative_numbers(terms_splitted);
    
    return negative_numbers
}

// Um überflüssige Leerzeichen zu entfernen
// Damit sind auch Strings in der Form " 3  +  5  * 3" möglich
fn remove_whitespaces(with_whitespaces: String) -> String {
    let result_string: String = with_whitespaces.chars().filter(|c| !c.is_whitespace()).collect();
    display_terminals("Leerzeichen entfernt".to_string(), &result_string);
    result_string
}

// Extra Funktionen für den Anfang einer Formel,
// und auf vollständigkeit der Klammern. Zudem wird hier ein
// bool zurückgegeben für die notwendigkeit der Klammerberechnung.
// Da hier nur eine Validierung gemacht wird benutze ich Referenzen
// anstatt die Ownership zu übergeben.
fn validation_brackets_operators(brackets_ops: &String) -> bool {
    let mut count_brackets: usize = 0;
    let brackets_ops_string: &String = brackets_ops;
    let mut if_brackets: bool = false;

    // Das allererste Zeichen wird gelesen.
    let first_char: char = brackets_ops_string.chars().nth(0).unwrap();

    // Prüfen ob das erste Zeichen des Strings korrekt
    // anfängt. Bei der Berechnung dürfen nur +, -, ( oder eine Zahl sein
    // den Anfang machen. Hier wird jeweils das Programm abgebrochen
    // um sicher zu gehen das es nicht in einen inkonsistenten 
    // Zusatnd kommt.
    if first_char == '-' || first_char == '+' || first_char.is_digit(10) || first_char == '(' {
        for terms in brackets_ops_string.chars() {
            if terms == ')' && count_brackets == 0 {
                panic!("Darf nicht mit einer ')' Klammer beginnen")
            }
            if terms == '(' || terms == ')' {
                count_brackets += 1;
                if_brackets = true;
            }
        }
        // prüfen ob die Klammern vollständig sind
        if count_brackets % 2 != 0 {
            panic!("Klammern nicht vollständig!")
        }
    }
    else {
        panic!("Anfang der Formel muss mit +,-, ( oder einer Zahl beginnen");
    }
    return if_brackets
    
}

// Mit dieser Funktion werden nur Leerzeichen vor den Operatoren und die Klammern
// gesetzt, um sie in der Funktion "split_terms" besser zu teilen.
fn terms_replace_operators(splitted_equation: String) -> String {
    let mut terms_replaced: String = String::new();

    for terms in splitted_equation.chars() {
            match terms {
                '(' => terms_replaced.push_str("( "),
                ')' => terms_replaced.push_str(" )"),
                '+' => terms_replaced.push_str( " + "),
                '-' => terms_replaced.push_str(" - "),
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

fn validate_negative_numbers(negative_numbers: Vec<String>) -> Vec<String> {
    let mut negative_num: Vec<String> = negative_numbers;
    
    let mut i = 0;
    while i < negative_num.len() {
        if negative_num[i].contains("-") && i > 0 && negative_num[i - 1] == "" {
            negative_num.remove(i - 1);
            negative_num.insert(i - 1, "0".to_string());
            i += 1;
        }
        i += 1;
    }
    return negative_num
}