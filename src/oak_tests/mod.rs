/*#![feature(plugin)]
#![plugin(oak)]

extern crate std;
extern crate oak_runtime;
use oak_runtime::*;

grammar! calc {
  #![show_api]

  /*program
    = spacing expression // modif
    / spacing command*/

  program = spacing expression

  command // modif
    = fn_kw identifier lparen (identifier)* rparen lcurly expression rcurly > create_function

  expression
    = term (term_op term)* > fold_left

  term
    = exponent (factor_op exponent)* > fold_left

  /*exponent
    = (factor exponent_op)* factor > fold_right*/

  exponent // modif
    = (element exponent_op)* element > fold_right

  element // modif
    = unary_op factor > fold_unary
    / factor

  factor
    = number > number_expr
    / identifier > variable_expr
    / let_expr > let_in_expr
    / lparen expression rparen

  let_expr = let_kw let_binding in_kw expression
  let_binding = identifier bind_op expression

  unary_op
    = neg_op > neg_bin_op // modif

  term_op
    = add_op > add_bin_op
    / sub_op > sub_bin_op

  factor_op
    = mul_op > mul_bin_op
    / div_op > div_bin_op

  exponent_op = exp_op > exp_bin_op

  identifier = !digit !keyword ident_char+ spacing > to_string
  ident_char = ["a-zA-Z0-9_"]

  digit = ["0-9"]
  number = digit+ spacing > to_number
  spacing = [" \n\r\t"]* -> (^)

  kw_tail = !ident_char spacing

  keyword = let_kw / in_kw / fn_kw // modif
  let_kw = "let" kw_tail
  in_kw = "in" kw_tail
  fn_kw = "fn" kw_tail // modif

  neg_op = "-" spacing //modif

  bind_op = "=" spacing
  add_op = "+" spacing
  sub_op = "-" spacing
  mul_op = "*" spacing
  div_op = "/" spacing
  exp_op = "^" spacing
  lparen = "(" spacing
  rparen = ")" spacing

  lcurly = "{" spacing // modif
  rcurly = "}" spacing // modif

  use std::str::FromStr;
  use self::Expression::*;
  use self::BinOp::*;
  use self::UnaOp::*;

  pub type PExpr = Box<Expression>;

  #[derive(Debug)]
  pub enum Expression {
    Variable(String),
    Number(u32),
    BinaryExpr(BinOp, PExpr, PExpr),
    UnaryExpr(UnaOp, PExpr),
    LetIn(String, PExpr, PExpr)
  }

  #[derive(Debug)]
  pub enum BinOp {
    Add, Sub, Mul, Div, Exp
  }

  #[derive(Debug)]
  pub enum UnaOp { // modif
    Neg
  }

  fn to_number(raw_text: Vec<char>) -> u32 {
    u32::from_str(&*to_string(raw_text)).unwrap()
  }

  fn number_expr(value: u32) -> PExpr {
    Box::new(Number(value))
  }

  fn variable_expr(ident: String) -> PExpr {
    Box::new(Variable(ident))
  }

  fn to_string(raw_text: Vec<char>) -> String {
    raw_text.into_iter().collect()
  }

  fn fold_left(head: PExpr, rest: Vec<(BinOp, PExpr)>) -> PExpr {
    rest.into_iter().fold(head,
      |accu, (op, expr)| Box::new(BinaryExpr(op, accu, expr)))
  }

  fn fold_right(front: Vec<(PExpr, BinOp)>, last: PExpr) -> PExpr {
    front.into_iter().rev().fold(last,
      |accu, (expr, op)| Box::new(BinaryExpr(op, expr, accu)))
  }

  fn fold_unary(una_op: UnaOp, expr: PExpr) -> PExpr { // modif
    Box::new(UnaryExpr(una_op, expr))
  }

  fn let_in_expr(var: String, value: PExpr, expr: PExpr) -> PExpr {
    Box::new(LetIn(var, value, expr))
  }

  fn create_function(name: String, params: Vec<(PExpr, BinOp)>, expression: PExpr) -> PExpr { // modif

  }

  fn add_bin_op() -> BinOp { Add }
  fn sub_bin_op() -> BinOp { Sub }
  fn mul_bin_op() -> BinOp { Mul }
  fn div_bin_op() -> BinOp { Div }
  fn exp_bin_op() -> BinOp { Exp }

  fn neg_bin_op() -> UnaOp { Neg } // modif
}


fn analyse_state(state: ParseState<StrStream, calc::PExpr>) {
    use oak_runtime::parse_state::ParseResult::*;
    match state.into_result() {
        Success(data) => println!("Full match: {:?}", data),
        Partial(data, expectation) => {
            println!("Partial match: {:?} because: {:?}", data, expectation);
        }
        Failure(expectation) => {
            println!("Failure: {:?}", expectation);
        }
    }
}

fn main() {
    analyse_state(calc::parse_program("2 * a".into_state())); // Complete
    analyse_state(calc::parse_program("2 *  ".into_state())); // Partial
    analyse_state(calc::parse_program("  * a".into_state())); // Erroneous

    analyse_state(calc::parse_program("-2".into_state())); // Complete modif
    analyse_state(calc::parse_program("-(2 * 4) + - 1".into_state())); // Complete modif

    let program1 =
        "let a = 5 in \
     let b = 2 in \
     a^2 + b^2 + (a - b)^2 \
    ";
    analyse_state(calc::parse_program(program1.into_state()));

    let program2 =
        "let a = \
       let b = 7^3 in 2 * b \
     in \
     a^2 - (let x = a in x * 2) \
    ";
    println!("{:?}", calc::parse_program(program2.into_state()).into_result());
}*/