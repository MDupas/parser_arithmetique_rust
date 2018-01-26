fn main() {
    let instruction = "3 + 1 - 21";
    let mut chars = instruction.chars();

    let num1 = 0;
    for c in chars.next() { // TODO : premier tour de boucle pour capter le 1er nombre
        let num2 = match c {
            ' ' => break,
            _ => match c.to_digit(10) {
                None => panic!("Devrait être un bombre !"),
                Some(n) => num1 * 10 + n,
            }
        };
    }
    let mut eva1 = Evaluable::Numb(num1);

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
            _ => panic!("Devrait être un opérateur !")
        };

        chars.next(); // Normalement, juste un espace ; TODO il peut ne pas y en avoir !

        let num2 = 0;
        for c in chars.next() { // 2e boucle pour 2e nombre
            let num2 = match c {
                ' ' => break,
                _ => match c.to_digit(10) {
                    None => panic!("Devrait être un bombre !"),
                    Some(n) => num2 * 10 + n,
                }
            };
        }

        let eva2 = Evaluable::Numb(num2);

        eva1 = Evaluable::Oper(BinaryOperator::new(eva1, eva2, op));

    }
    let result = match eva1 {
        Evaluable::Numb(numb) => numb,
        Evaluable::Oper(oper) => oper.eval(),
    };
    println!("Résultat : {}", result);
}

enum Evaluable {
    Numb(u32),
    Oper(BinaryOperator)
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

    fn new(a: Evaluable, b: Evaluable, op: Operator) -> BinaryOperator {
        BinaryOperator {
            a,
            b,
            op
        }
    }

    fn a(&self) -> Evaluable { // Pourrais juste mettre les fields public
        self.a
    }

    fn b(&self) -> Evaluable {
        self.b
    }

    fn eval(&self) -> u32 {
        let x = match &self.a {
            Evaluable::Numb(num) => num,
            Evaluable::Oper(&bin_op) => &bin_op.eval()
        };
        let y = match &self.b {
            Evaluable::Numb(num) => num,
            Evaluable::Oper(&bin_op) => &bin_op.eval()
        };
        match &self.op {
            Operator::Add => x + y,
            Operator::Min => x - y,
            Operator::Mul => x * y,
            Operator::Div => x / y
        }
    }
}