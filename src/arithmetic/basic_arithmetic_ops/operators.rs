
pub(crate) mod operator {
    #[derive(Debug)]
    pub enum Operators {
        Pow,
        Mult,
        Divide,
        Add,
        Sub,
    }

    impl Operators {
        // Eine Methode, um den Operator anhand des Symbols zu finden
        
        fn from_symbol(symbol: &str) -> Option<Operators> {
            match symbol {
                "^" => Some(Operators::Pow),
                "*" => Some(Operators::Mult),
                "/" => Some(Operators::Divide),
                "+" => Some(Operators::Add),
                "-" => Some(Operators::Sub),
                _ => None,  // Unbekanntes Symbol
            }
        }

        // Methode, um das Zeichen eines Operators zu bekommen
        fn symbol(&self) -> &str {
            match self {
                Operators::Pow => "^",
                Operators::Mult => "*",
                Operators::Divide => "/",
                Operators::Add => "+",
                Operators::Sub => "-",
            }
        }
    }

    // Wandelt ein Symbol in einen Operator um.
    // Gibt `Some(Operator)` zurück, wenn das Symbol bekannt ist, ansonsten `None`.
    pub fn get_operator(symbol: &str) -> String {
        let ops: Option<Operators> = Operators::from_symbol(symbol);
        let op: String;

        match ops {
                Some(operator) =>  op = operator.symbol().to_string(),
                None => op = "Operator nicht gefunden".to_string(),
        }
        return op
    }

    pub fn pow_operator() -> String {
        return "^".to_string()
    }

    pub const fn mult_operator() -> Operators {
        Operators::Mult
    }

    pub const fn divide_operator() -> Operators {
        Operators::Divide
    }

    pub const fn add_operator() -> Operators {
        Operators::Add
    }

    pub const fn sub_operator() -> Operators {
        Operators::Sub
    }
}
