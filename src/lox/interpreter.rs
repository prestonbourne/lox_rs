use super::expr;

#[derive(Debug)]
enum Value {
    // Value fields here
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub struct Interpreter {
    // Interpreter fields here
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(expr: &expr::Expr)  {
        match Interpreter::interpret_expr(expr) {
            Ok(val) => println!("{:?}", val),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn interpret_expr(expr: &expr::Expr) -> Result<Value, String> {
        match expr {
            expr::Expr::Literal(lit) => Ok(Interpreter::interpret_literal(lit)),
            expr::Expr::Binary(left, op, right) => {
                let val = Interpreter::interpret_binary(left, *op, right)?;
                Ok(val)
            }
            expr::Expr::Grouping(group) => Interpreter::interpret_expr(group),
            expr::Expr::Unary(op, expr) => {
                let val = Interpreter::interpret_unary(*op, expr)?;
                Ok(val)
            }
            _ => todo!("Not implemented"),
        }
    }

    fn interpret_literal(lit: &expr::Literal) -> Value {
        match lit {
            expr::Literal::Number(n) => Value::Number(*n),
            expr::Literal::String(s) => Value::String(s.clone()),
            expr::Literal::Boolean(true) => Value::Bool(true),
            expr::Literal::Boolean(false) => Value::Bool(false),
            expr::Literal::Nil => Value::Nil,
        }
    }

    fn interpret_unary(op: expr::UnaryOp, expr: &expr::Expr) -> Result<Value, String> {
        let val = Interpreter::interpret_expr(expr)?;

        match (op.ty, &val) {
            (expr::UnaryOpType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (expr::UnaryOpType::Bang, _) => Ok(Value::Bool(!Interpreter::is_truthy(&val))),
            (_, _) => Err(Interpreter::invalid_unary_operand(&op, &val)),
        }
    }

    fn interpret_binary(
        left: &expr::Expr,
        op: expr::BinaryOp,
        right: &expr::Expr,
    ) -> Result<Value, String> {
        let left_val = Interpreter::interpret_expr(left)?;
        let right_val = Interpreter::interpret_expr(right)?;

        match (op.ty, &left_val, &right_val) {
            (expr::BinaryOpType::Minus, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Number(l - r))
            }
            (expr::BinaryOpType::Slash, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Number(l / r))
            }
            (expr::BinaryOpType::Star, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Number(l * r))
            }
            (expr::BinaryOpType::Plus, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Number(l + r))
            }
            (expr::BinaryOpType::Plus, Value::String(l), Value::String(r)) => {
                return Ok(Value::String(l.to_owned() + r))
            }
            (expr::BinaryOpType::Greater, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Bool(l > r))
            }
            (expr::BinaryOpType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Bool(l >= r))
            }
            (expr::BinaryOpType::Less, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Bool(l < r))
            }
            (expr::BinaryOpType::LessEqual, Value::Number(l), Value::Number(r)) => {
                return Ok(Value::Bool(l <= r))
            }
            (expr::BinaryOpType::EqualEqual, _, _) => {
                return Ok(Value::Bool(Interpreter::is_equal(&left_val, &right_val)))
            }
            (expr::BinaryOpType::NotEqual, _, _) => {
                return Ok(Value::Bool(!Interpreter::is_equal(&left_val, &right_val)))
            }
            (_, _, _) => {
                return Err(Interpreter::invalid_binary_operand(
                    &op, &left_val, &right_val,
                ))
            }
        };
    }

    // utils
    // fn checkNumberOperand(op: &expr::UnaryOp, operand: &Value) -> Result<(), String> {
    //     if let Value::Number(_) = operand {
    //         return Ok(());
    //     }
    //     Err(format!("Operand must be a number: {:?}", op))
    // }

    // fn checkNumberOperands(op: &expr::BinaryOp, left: &Value, right: &Value) -> Result<(), String> {
    //     if let (Value::Number(_), Value::Number(_)) = (left, right) {
    //         return Ok(());
    //     }
    //     Err(format!("Operands must be numbers: {:?}", op))
    // }

    fn invalid_binary_operand(op: &expr::BinaryOp, left: &Value, right: &Value) -> String {
        format!(
            "Invalid operands for binary operation: {:?}: {:?} {:?}",
            op, left, right
        )
    }

    fn invalid_unary_operand(op: &expr::UnaryOp, operand: &Value) -> String {
        format!(
            "Invalid operand for unary operation: {:?}: {:?}",
            op, operand
        )
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            _ => true,
        }
    }

    fn is_equal(left: &Value, right: &Value) -> bool {
        match (left, right) {
            // basics
            (Value::Nil, Value::Nil) => true,
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,

            // lox specific cast strings to numbers and compare
            (Value::Number(l), Value::String(r)) => {
                if let Ok(r) = r.parse::<f64>() {
                    return l == &r;
                }
                false
            }
            (Value::String(l), Value::Number(r)) => {
                if let Ok(l) = l.parse::<f64>() {
                    return &l == r;
                }
                false
            }

            (_, _) => false,
        }
    }
}
