// Attribution 4.0 International (CC BY 4.0)
// more on https://creativecommons.org/licenses/by/4.0/#

pub use super::Operator;

extern crate std;

pub fn search_number( chars: &mut std::str::Chars) -> i32 {
    let mut num : i32 = 0;
    loop {
        let c = match chars.next() {
            None => break,
            Some(ch) => ch,
        };
        num = match c {
            ' ' => break,
            _ => match c.to_digit(10) {
                None => panic!("Devrait être un nombre !\nÉtait un \"{}\"", c),
                Some(n) => num * 10 + (n as i32),
            }
        };
    };
    num
}

pub fn search_operator( chars: &mut std::str::Chars) -> Result<Operator,String> {
    loop {
        let c = match chars.next() {
            None => break,
            Some(new_c) => new_c,
        };
        let op = match c { // un seul match pour l'opérateur
            '+' => Operator::Add,
            '-' => Operator::Min,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            ' ' => continue,
            _ => panic!("Devrait être un opérateur !\nÉtait un \"{}\"", c)
        };
        return Ok(op)
    }
    Err("Plus de caracteres".to_string())
}