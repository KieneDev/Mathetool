use crate::{arithmetic::basic_arithmetic_ops::brackets_solver::calculate_resolve_brackets, paths::calc_nums::calculate_numbers_powers};
pub(crate) use crate::paths::calc_nums::{calculate_numbers_addition, calculate_numbers_mult_diff};


// Rechenregeln von Klammer, Potenzen, Punkt vor Strich und zu Addition und Subtraktion,
// von links nach rechts brauche ich nicht da mein Tool das sowieso
// macht.
pub fn rules_for_calculation(formula: Vec<String>) -> String {

    let found_ops: Vec<OperatorInfo> = find_operators(&formula);
    
    let mut changed_formula: Vec<String> = formula;

    if found_ops[0].active {
        println!("Klammern auflösen/berechnen\n");
        changed_formula = calculate_resolve_brackets(changed_formula);
        println!("--------------------------");
    }

    if found_ops[1].active {
        println!("Potenzen werden berechnet.\n");
        changed_formula = calculate_numbers_powers(changed_formula);
        println!("--------------------------");
    }
    if found_ops[2].active || found_ops[3].active {
        println!("Berechnung (Punkt vor Strich)\n");
        changed_formula = calculate_numbers_mult_diff(changed_formula);
        println!("--------------------------");
    }

    if found_ops[4].active || found_ops[5].active {
        println!("Berechnung (Plus und Minus)\n");
        changed_formula = calculate_numbers_addition(changed_formula);
        println!("--------------------------");
    }
    
    return changed_formula[0].to_string()
}

pub struct OperatorInfo {
    index: usize,
    symbol: char,
    active: bool,
}

// Getter um auf die Werte lesend zu zugreifen
impl OperatorInfo {
    // Getter für index
    pub fn index(&self) -> usize {
        self.index
    }

    // Getter für symbol
    pub fn symbol(&self) -> &char {
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
    
    let mut operations:Vec<OperatorInfo> = vec![
        OperatorInfo { index: 0, symbol: '(', active: false },
        OperatorInfo { index: 1, symbol: '^', active: false },
        OperatorInfo { index: 2, symbol: '*', active: false },
        OperatorInfo { index: 3, symbol: '/', active: false },
        OperatorInfo { index: 4, symbol: '+', active: false },
        OperatorInfo { index: 5, symbol: '-', active: false },
    ];

    for i in numbers.iter() {

        if operations[0].active && operations[1].active && operations[2].active && operations[3].active 
        && operations[4].active && operations[5].active {
            break;
        }

        if i.contains("(") {
            if operations[0].active == true { continue; }
            operations[0].active = true;
        }

        if i.contains("^") {
            if operations[1].active == true { continue; }
            operations[1].active = true;
        }
        if i.contains("*") {
            if operations[2].active == true { continue; }
            operations[2].active = true;
        }
        if i.contains("/") {
            if operations[3].active == true { continue; }
            operations[3].active = true;
        }
        if i.contains("+") {
            if operations[4].active == true { continue; }
            operations[4].active  = true;
        }
        if i.contains("-") {
            if operations[5].active == true { continue; }
            operations[5].active = true;
        }
    }
    return operations
}
