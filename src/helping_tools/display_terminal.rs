pub(crate) fn display_terminals(output_string: String, formula: &String) {
    let width: usize = 30;

    println!("{:<width$} : {}", output_string, formula, width = width);
}

pub(crate) fn display_terminals_validate(output_string: String, left: &String, right: &String, operator: &String, result: &String) {
    let width: usize = 33;

    println!("{:<width$} : {} {} {} = {}", output_string, left, operator, right, result, width = width);
}

pub fn clearscreen() {
    #[cfg(unix)]
    {
        let _ = std::process::Command::new("clear").status();
    }

    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd").args(&["/C", "cls"]).status();
    }
}