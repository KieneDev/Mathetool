use super::display_terminal::display_terminals;

// Diese Funktion ist für andere sichtbar und macht die
// Stringmanipulationen verfügbar.
pub fn strings_refactor(crazy_string: String) -> Vec<String> {
    let mut crazy_string_refactor: String = crazy_string;

    remove_whitespaces(&mut crazy_string_refactor);
    validation_brackets_operators(&mut crazy_string_refactor);
    terms_replace_operators(&mut crazy_string_refactor);

    let terms_splitted: Vec<String> = split_terms(crazy_string_refactor);
    let negative_numbers: Vec<String> = validate_negative_numbers(terms_splitted);
    
    return negative_numbers
}

// Um überflüssige Leerzeichen zu entfernen
// Damit sind auch Strings in der Form " 3  +  5  * 3" möglich
fn remove_whitespaces(with_whitespaces: &mut String) {
    let result_string: String = with_whitespaces.chars().filter(|c| !c.is_whitespace()).collect();

    display_terminals("Leerzeichen entfernt".to_string(), &result_string);

    *with_whitespaces = result_string;
}

// Extra Funktionen für den Anfang einer Formel,
// und auf Vollständigkeit der Klammern.
fn validation_brackets_operators(brackets_ops: &mut String) {
    let mut count_brackets: usize = 0;
    //let brackets_ops_string: &String = brackets_ops;

    // Das allererste Zeichen wird gelesen.
    let first_char: char = brackets_ops.chars().nth(0).unwrap();

    // Prüfen ob das erste Zeichen des Strings korrekt
    // anfängt. Bei der Berechnung dürfen nur +, -, ( oder eine Zahl
    // den Anfang machen. Hier wird jeweils das Programm abgebrochen
    // um sicher zu gehen das es nicht in einen inkonsistenten 
    // Zusatnd kommt.
    if first_char == '-' || first_char == '+' || first_char.is_digit(10) || first_char == '(' {
        for terms in brackets_ops.chars() {
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
        panic!("Anfang der Formel muss mit +,-, ( oder einer Zahl beginnen");
    }
}

// Mit dieser Funktion werden nur Leerzeichen vor den Operatoren und die Klammern
// gesetzt, um sie in der Funktion "split_terms" besser zu teilen.
fn terms_replace_operators(splitted_equation: &mut String) {
    let mut terms_replaced: String = String::new();

    for terms in splitted_equation.chars() {
            match terms {
                '(' => terms_replaced.push_str("( "),
                ')' => terms_replaced.push_str(" )"),
                '+' => terms_replaced.push_str( " + "),
                '-' => terms_replaced.push_str(" - "),
                '*' => terms_replaced.push_str(" * "),
                ':' => terms_replaced.push_str(" : "),
                '^' => terms_replaced.push_str(" ^ "),
                '/' => terms_replaced.push_str(" / "),
                _ => terms_replaced.push(terms),
        }
    }
    display_terminals("Leerzeichen vor den Operatoren".to_string(), &terms_replaced);
    *splitted_equation = terms_replaced;
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

// Wenn die Formel mit einem Minus anfängt, wird hier vorne eine 0 rangehängt,
// damit die Berechnung sauber durchgeführt wird. Da die Berechnungen bei allen auf 
// dem Prinzip "Links-Operator-Rechts" ausgeführt wird. Auch wird hier geparst wenn
// "-(-5)" auftritt. Auch die Berechnung mit negativen Zahlen wird hier geparst.
fn validate_negative_numbers(negative_numbers: Vec<String>) -> Vec<String> {
    let mut negative_num: Vec<String> = negative_numbers;
    
    let mut i = 0;
    while i < negative_num.len() {
        // Fallunterscheidung wenn (-5) auftritt
        if negative_num[i].contains("-") && i > 0 && negative_num[i - 1] == "" {
            negative_num.remove(i - 1);
            negative_num.insert(i - 1, "0".to_string());
            i += 1;
        }
        else if i == 0 && negative_num[i].contains("-") {
            // Sonderfall wenn die Klammer mit - anfängt.
            if negative_num[i].contains("-") && negative_num[i + 1] == "(" {
                negative_num.insert(i, "0".to_string());
                continue;
            }
            let negative_number = 0 - negative_num[i + 1].parse::<i32>().expect("invalid number");
            negative_num.remove(i);
            negative_num.insert(i, negative_number.to_string());
            negative_num.remove(i + 1);
        }
        i += 1;
    }
    return negative_num
}