// use crate::arithmetic::basic_arithmetic_ops::index_operators::find_index;


// // Diese Funktionen sind erstmal auf Eis gesetzt, weil es in meiner 
// // derzeitigen Entwicklung keinen richtigen Sinn macht, wenn ich zur
// // Formelumstellung oder Formeln komme werde ich diese Funktionen als
// // eigenes Modul für die Bruchrechnung verwenden und weiter ausbauen.
// #[warn(unused_assignments)]
// pub fn calculate_fraction(fractions: Vec<String>) -> Vec<String> {
//     let mut result_fraction: Vec<String> = fractions;
//     let mut index_brackets: Vec<(usize, usize)> = Vec::new();
//     let mut fractions_term: Vec<Bruch> = vec![];

//     println!("Vektor davor: {:?}", result_fraction);
//     parse_fractions(&mut result_fraction, &mut fractions_term);
//     println!("Vektor danach: {:?}", result_fraction);

//     index_brackets = find_index(&result_fraction);
//     println!("Index der Klammern: {:?}", index_brackets);

//     calculate_rules_fraction(&mut result_fraction, &mut fractions_term, &mut index_brackets);

//     //todo!("Noch im Bau! (calculate_fraction)");
//     return Vec::new()
// }

// struct Bruch {
//     numerator: i32,
//     denominator: i32,
// }

// // hier werden alle Brüche in einem Vektor gespeichert
// fn parse_fractions(fractions: &mut Vec<String>, fractions_term: &mut Vec<Bruch>) {
//     let mut index: usize = 0;

//     while index < fractions.len()  {
//         if fractions[index].contains("/") {
//             if fractions[index + 1] == "0" {
//                 panic!("Division by Zero!")
//             }

//             let vec_fractions = Bruch {
//                 numerator: fractions[index - 1].parse::<i32>().expect("invalid numerator"),
//                 denominator: fractions[index + 1].parse::<i32>().expect("invalid numerator"),
//             };

//             fractions_term.push(vec_fractions);

//             // Hier werden die Brüche durch einen Marker ersetzt.
//             let index_bruch: String = "bruch_".to_string() + &((fractions_term.len()) - 1).to_string();
//             fractions.remove(index);
//             fractions.insert(index, index_bruch);
//             fractions.remove(index + 1);
//             fractions.remove(index - 1);
            
//         }
//         index += 1;
//     }
// }

// fn calculate_rules_fraction(fractions: &mut Vec<String>, fractions_term: &mut Vec<Bruch>, index_brackets: &mut Vec<(usize, usize)>) {
//     let mut index: usize = 0;

//     while index < fractions.len() {
        
        
//         index += 1;
//     }

//     todo!("Noch im Bau (calculates_rules_fraction)!")
// }