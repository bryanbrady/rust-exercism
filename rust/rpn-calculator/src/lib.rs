#[derive(Debug,Clone,Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn to_value(self) -> Option<i32> {
        match self {
            CalculatorInput::Value(v) => Some(v),
            _ => None
        }
    }

    fn compute(self, a: CalculatorInput, b: CalculatorInput) -> Option<CalculatorInput> {
        let a = a.to_value().unwrap();
        let b = b.to_value().unwrap();
        let out = match self {
            CalculatorInput::Add => a + b,
            CalculatorInput::Subtract => a - b,
            CalculatorInput::Multiply => a * b,
            CalculatorInput::Divide => a / b,
            _ => return None
        };
        Some(CalculatorInput::Value(out))
    }
}


type Expr = CalculatorInput;

pub fn evaluate(inputs: &[Expr]) -> Option<i32> {
    let mut stack: Vec<Expr> = Vec::new();

    for e in inputs {
        match e {
            Expr::Value(_) => stack.push(*e),
            _ => {
                let b = stack.pop();
                let a = stack.pop();
                match (a, b) {
                    (Some(a), Some(b)) => stack.push(e.compute(a,b).unwrap()),
                    _ => return None
                }
            }
        }
    }

    match stack.len() {
        1 => stack.pop().unwrap().to_value(),
        _ => None
    }
}
