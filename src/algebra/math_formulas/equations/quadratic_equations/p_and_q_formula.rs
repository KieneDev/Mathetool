use super::display_results;

/*
 * Quadratische Gleichung lösen mit der pq-Formel
 *
 * Diese Funktion berechnet die Lösungen einer quadratischen Gleichung
 * der Form ax² + bx + c = 0 mit Hilfe der pq-Formel.
 *
 * WICHTIG:
 * - Die pq-Formel setzt voraus, dass die Gleichung in der Normalform
 *   x² + px + q = 0 vorliegt. Falls a ≠ 1 ist, muss vorher durch a geteilt werden.
 * - Der Radikand muss mit 4 multipliziert werden, um mit der Mitternachtsformel
 *   vergleichbar zu sein.
 *
 * Parameter:
 * - `variables_array_coeffizenten`: Ein Array mit drei Elementen [a, b, c], das die Koeffizienten
 *   der quadratischen Gleichung enthält.
 *
 * Berechnungsschritte:
 * 1. Falls a ≠ 1, wird die Gleichung in die Normalform umgewandelt (p = b/a, q = c/a).
 * 2. Die Diskriminante D wird berechnet: D = ((p/2)² - q) * 4
 * 3. Je nach Wert der Diskriminante gibt es:
 *    - Zwei Lösungen (D > 0)
 *    - Eine Lösung (D = 0)
 *    - Komplexe Lösungen (D < 0)
 * 4. Wenn man es auch genau sagen will wird der Wert der Diskriminante als Radikand bezeichnet
 *
 * Hinweise:
 * - Falls die Diskriminante negativ ist, gibt es keine reellen Lösungen.
 * - Komplexe Lösungen werden in der Form a ± bi ausgegeben.
 *
 * Beispiel:
 * - Eingabe: [2, 3, 5]
 * - Umformung: x² + (3/2)x + (5/2) = 0
 * - Diskriminante: -11 * 4 = -44
 * - Ergebnis: Komplexe Lösungen
 */

pub fn p_and_q_formula(variables_array_coeffizenten: [f64; 3]) {
    println!("Quadratische Gleichung mittels pq-Formel lösen!");

    let mut coeffizient_p: f64 = variables_array_coeffizenten[1];
    let mut coeffizient_q: f64 = variables_array_coeffizenten[2];

    let radikand: f64 = ((coeffizient_p/2.0)*(coeffizient_p/2.0))-coeffizient_q;
    
    let mut result: f64;

    let mut temp_variable_a: f64 = variables_array_coeffizenten[0];

    // Achtung bei der pq-Formel ist es wichtig vorher zu prüfen
    // ob auch die Normalform vorliegt, heißt der Koeffizient a
    // ist ungleich 1 dann muss umgeformt werden, dabei gibt a den Teiler
    // an.
    if temp_variable_a != 1.0 {

        // ermitteln der Variablen p und q, um sie von der allgemeinen Form
        // in die Normalform zu bringen.
        coeffizient_p = variables_array_coeffizenten[1]/variables_array_coeffizenten[0];
        coeffizient_q = variables_array_coeffizenten[2]/variables_array_coeffizenten[0];

        println!("\nUmgestellt zur Normalform\n");
        temp_variable_a = 1.0;
    }
    if temp_variable_a == 1.0 {
        
        // Hier muss man wissen das der Radikand noch mal mit 4.0
        // mutlipliziert sollte um von einem Bruch auch eine Natürliche Zahl
        // zu kommen.
        println!("\nRadikand = {}\n", radikand*4.0);

        // Berechnung mit komplexen Zahlen "i"
        if radikand < 0.0 {

            result = -(coeffizient_p / 2.0);
            let result_imag: f64 = (coeffizient_q - ((coeffizient_p/2.0)*(coeffizient_p/2.0))).sqrt();

            display_results::display_result(&result, &radikand, Some(1), Some(result_imag));
        }
        else if radikand == 0.0 {
            println!("Es gibt genau eine Lösung!");

            result = -(coeffizient_p / 2.0);
            display_results::display_result(&result, &radikand, None, None);
        }
        else {
            println!("Es gibt 2 Lösungen!");

            result = -(coeffizient_p / 2.0) + radikand.sqrt();
            display_results::display_result(&result, &radikand, Some(1), None);

            result = -(coeffizient_p / 2.0) - radikand.sqrt();
            display_results::display_result(&result, &radikand, Some(0), None);
        }
    }
}