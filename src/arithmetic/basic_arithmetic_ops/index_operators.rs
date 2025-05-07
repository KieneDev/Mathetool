
pub fn find_index(brackets: &Vec<String>) -> Vec<(usize, usize)> {
    return get_index_brackets_operators(&brackets)
}

// Hier werden die Klammerpaare mit einem Stack zu einem
// Vektor erzeugt. Somit hat man alle Klammerpaare sauber vom
// inneren zum äusseren, und alle einzelnen.
fn get_index_brackets_operators(brackets: &Vec<String>) -> Vec<(usize, usize)> {
    let mut stack: Vec<usize> = Vec::new();
    let mut bracket_pairs: Vec<(usize, usize)> = Vec::new(); // Fertige Klammerpaare

    for (index, value) in brackets.iter().enumerate() {
        if value == "(" {
            stack.push(index);
        } 
        else if value == ")" {
            if let Some(open_index) = stack.pop() {
                bracket_pairs.push((open_index, index));
            } 
        }
    }
    return bracket_pairs
}