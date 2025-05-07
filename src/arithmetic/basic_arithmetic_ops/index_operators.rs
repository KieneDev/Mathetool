
pub fn find_index(brackets: &Vec<String>) -> Vec<(usize, usize)> {
    return get_index_brackets_operators(&brackets)
}

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

fn get_index_mult_div(mult_div: Vec<String>) -> Vec<usize> {
    let mut index_mult_div: Vec<usize> = Vec::new(); 
    
    for (index, operator) in mult_div.iter().enumerate() {
        if operator.contains("*") || operator.contains("/") {
            index_mult_div.push(index);
        }
    }
    return index_mult_div
}