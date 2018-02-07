// Attribution 4.0 International (CC BY 4.0)
// more on https://creativecommons.org/licenses/by/4.0/#

pub enum Evaluable {
    Numb(i32),
    Oper(Box<BinaryOperator>)
}

pub enum Operator {
    Add,
    Min,
    Mul,
    Div
}

pub struct BinaryOperator {
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