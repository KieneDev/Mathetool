#![allow(dead_code)]

use menue::mathtool_menue_terminal;

mod paths;

mod helping_tools;
mod algebra;
mod arithmetic;
mod menue;

fn main() {
    //let equation_string: String = "(100+2)^2-2*(5*(8/(9-12)+5)/2)*8+1*(2+5)*9".to_string();
    // let equation_string: String = "100+2*2-2^5*8/9-12+5/2*8+12".to_string();
    //let equation_string: String = "(3+2)^2 * (4 ^ (-3)) - 5 * (10 / (2 + 3)) + 8 ^ 2".to_string();
    //let equation_string: String = "-(5-5)".to_string();
    //let equation_string: String = "-4 + 5/6 * (3/2 + 6 / 4) + 6 : (3/9 * 3/4 + 1)".to_string();

    mathtool_menue_terminal();
}