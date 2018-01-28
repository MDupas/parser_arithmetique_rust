fn main() {
    let instruction = "3 + 1 - 21";
    let mut chars = instruction.chars();
    let mut num1 : i32 = 0;
    let mut i = 0;

    println!("taille : {}", chars.clone().count());
    loop { // TODO : fonction (pour éviter duplication / apprendre des trucs)
        let c = match chars.next() {
            None => panic!("L'instruction de calcul est vide"),
            Some(ch) => ch,
        };
        println!("index {}", i);
        i += 1;
        num1 = match c {
            ' ' => break,
            _ => match c.to_digit(10) {
                None => panic!("Devrait être un nombre !"),
                Some(n) => num1 * 10 + (n as i32),
            }
        };
        println!("testeuh");
    }
    println!("1er nombre : {}", num1);
    let mut eva1 = Evaluable::Numb(num1);

    loop {
        println!("taille : {}", chars.clone().count());
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

        let mut num2 = 0;
        loop { // 2e boucle pour 2e nombre
            let c = match chars.next() {
                None => break,
                Some(ch) => ch,
            };
            num2 = match c {
                ' ' => break,
                _ => match c.to_digit(10) {
                    None => panic!("Devrait être un nombre !\nÉtait un \"{}\"", c),
                    Some(n) => num2 * 10 + (n as i32),
                }
            };
        }

        let eva2 = Evaluable::Numb(num2);

        eva1 = Evaluable::Oper(Box::new(BinaryOperator::new(eva1, eva2, op)));

    }
    let result = match eva1 {
        Evaluable::Numb(numb) => numb,
        Evaluable::Oper(oper) => (*oper).eval(),
    };
    println!("Résultat : {}", result);
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

    pub fn a(&self) -> &Evaluable { // Pourrais juste mettre les fields public
        &self.a
    }

    pub fn b(&self) -> &Evaluable {
        &self.b
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