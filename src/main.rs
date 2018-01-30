fn main() {
    let instruction = "3 + 2 - 2 * 10 / 2";
    let mut chars = instruction.chars();

    println!("Instruction : {} ", instruction);

    let num1 = search_number(&mut chars);
    let mut eva1 = Evaluable::Numb(num1);

    loop {
        //println!("taille : {}", chars.clone().count());

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

        chars.next(); // Normalement, juste un espace ; TODO il peut ne pas y en avoir !

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

fn search_number( chars: &mut std::str::Chars) -> i32 {
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

enum Evaluable {
    Numb(i32),
    Oper(Box<BinaryOperator>)
}

enum Operator {
    Add,
    Min,
    Mul,
    Div
}

struct BinaryOperator {
    a: Evaluable,
    b: Evaluable,
    op: Operator
}

impl BinaryOperator {

    pub fn new(a: Evaluable, b: Evaluable, op: Operator) -> BinaryOperator {
        BinaryOperator {
            a,
            b,
            op
        }
    }

    pub fn eval(&self) -> i32 {
        let x = match &(self.a) {
            &Evaluable::Numb(num) => num,
            &Evaluable::Oper(ref bin_op) => (*bin_op).eval()
        };
        let y = match &(self.b) {
            &Evaluable::Numb(num) => num,
            &Evaluable::Oper(ref bin_op) => (*bin_op).eval()
        };
        match self.op {
            Operator::Add => x + y,
            Operator::Min => x - y,
            Operator::Mul => x * y,
            Operator::Div => x / y
        }
    }
}