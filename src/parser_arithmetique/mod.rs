// Attribution 4.0 International (CC BY 4.0)
// more on https://creativecommons.org/licenses/by/4.0/#

pub use self::ast::Operator;
pub use self::ast::Evaluable;
pub use self::ast::BinaryOperator;
mod ast;

pub use self::parser::*;
mod parser;

extern crate std;

pub fn main() {
    let instruction = "3 + 2 - 2 * 10 / 2";
    let mut chars = instruction.chars();

    println!("Instruction : {} ", instruction);

    let num1 = search_number(&mut chars);
    let mut eva1 = Evaluable::Numb(num1);

    loop {
        //println!("taille : {}", chars.clone().count());

        let op = match search_operator(&mut chars) {
            Ok(o) => o,
            Err(_) => break
        };

        chars.next(); // Normalement, juste un espace ; TODO il peut ne pas y en avoir ! ; chars.peekable()

        let num2 = search_number(&mut chars);

        let eva2 = Evaluable::Numb(num2);

        eva1 = Evaluable::Oper(Box::new(BinaryOperator::new(eva1, eva2, op)));

    }
    let result = match eva1 {
        Evaluable::Numb(numb) => numb,
        Evaluable::Oper(oper) => (*oper).eval(),
    };
    println!("Résultat : {}", result);
}