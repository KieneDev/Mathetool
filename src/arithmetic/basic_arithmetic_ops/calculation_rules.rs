pub(crate) use crate::paths::calc_nums::{calculate_numbers_addition, calculate_numbers_mult_diff, calculate_resolve_brackets};

use super::operators::operator::get_operator;

// Rechenregeln von Klammer, Potenzen, Punkt vor Strich und zu Addition und Subtraktion,
// von links nach rechts brauche ich nicht da mein Tool das sowieso
// macht.
pub fn rules_for_calculation(formula: Vec<String>) -> Vec<OperatorInfo> {

    let found_ops: Vec<OperatorInfo> = find_operators(&formula);
    //let founds_ops_clone: Vec<OperatorInfo> = found_ops.iter().copied().clone;
    let mut changed_formula: Vec<String> = formula;
    let mut last_calculate: String = String::new();

    // if found_ops[0].1 {
    //     println!("Klammern berechnen und auflösen.\n");
    //     changed_formula = calculate_resolve_brackets(changed_formula, operator_count[0]);
    //     println!("--------------------------");
    // }
    // if found_ops[0].active {
    //     println!("Potenzen werden berechnet.\n");
    //     changed_formula = calculate_numbers_powers(changed_formula);
    //     println!("--------------------------");
    // }
    // if found_ops[0].active {
    //     println!("Berechnung (Punkt vor Strich)\n");
    //     changed_formula = calculate_numbers_mult_diff(changed_formula);
    //     println!("--------------------------");

    //     if let Some(temp) = operator_count.get_mut(2) {*temp = 0;}
    // }
    // if operator_count[3] > 0 {
    //     println!("Berechnung (Addition und Subtraktion)\n");
    //     last_calculate = calculate_numbers_addition(changed_formula);
    //     println!("--------------------------");
    // }
      
    return found_ops
    // println!("Endergebnis {:.2}", last_calculate.parse::<f64>().expect("Invalid Number"));
    // println!("Erfolg!");
}

pub struct OperatorInfo {
    index: usize,
    symbol: String,
    active: bool,
}

// Getter um auf die Werte lesend zu zugreifen
impl OperatorInfo {
    // Getter für index
    pub fn index(&self) -> usize {
        self.index
    }

    // Getter für symbol
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    // Getter für active
    pub fn active(&self) -> bool {
        self.active
    }
}

// Hier werden alle Operatoren gezählt, wichtig um zu wissen
// ob eine Funktion zum berechnen aufgerufen werden muss 
// oder auch nicht. Und damit die Rechenregeln in der richtigen
// Reihenfolge ablaufen.
fn find_operators(numbers: &Vec<String>) -> Vec<OperatorInfo> {
    
    let mut operations: Vec<OperatorInfo> = Vec::new();

    for i in numbers.iter() {
        if i.contains("^") && !operations[0].active {
            operations.push(OperatorInfo { index: 0, symbol: get_operator(i), active: true });
        }
        else if i.contains("*") && !operations[1].active {
            operations.push(OperatorInfo { index: 1, symbol: get_operator(i), active: true });
        }
        else if i.contains("/") && !operations[2].active {
            operations.push(OperatorInfo { index: 2, symbol: get_operator(i), active: true });
        }
        else if i.contains("+") && !operations[3].active{
            operations.push(OperatorInfo { index: 3, symbol: get_operator(i), active: true });
        }
        else if i.contains("-") && !operations[4].active {
            operations.push(OperatorInfo { index: 4, symbol: get_operator(i), active: true });
        }
    }
    return operations
}
