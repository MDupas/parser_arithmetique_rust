fn main() {
    // Boucle
        //2 - Lire un caractère
        //3 - Déterminer quoi faire du caractère (augmenter chiffre, opération)
        //4 - Construire AST (on s'en fout des prios)

    let instruction = "3 + 1 - 21";


}

enum Evaluable {
    Numb(int),
    Oper(Ast)
}

enum Operator {
    Add,
    Min,
    Mul,
    Div
}

struct Ast {
    a: Evaluable,
    b: Evaluable,
    op: Operator
}

impl Ast {
    fn set_a(&self, new_a: Evaluable) -> &Ast {
        a = newA;
        &self
    }

    fn set_b(&self, new_b: Evaluable) -> &Ast {
        b = newB;
        &self
    }

    fn eval(&self) -> int {
        x = match &self.a {
            Evaluable::Numb(num) => num,
            Evaluable::Oper(&ast) => &ast.eval()
        };
        y = match &self.b {
            Evaluable::Numb(num) => num,
            Evaluable::Oper(&ast) => &ast.eval()
        };
        match &self.op {
            Operator::Add => x + y,
            Operator::Min => x - y,
            Operator::Mul => x * y,
            Operator::Div => x / y
        }
    }
}