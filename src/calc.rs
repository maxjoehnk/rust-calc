#[derive(Debug)]
pub enum Actions {
    ADD,
    SUBTRACT,
    DIVIDE,
    MULTIPLY,
    POWER,
}

pub enum Value {
    Primitive(f64),
    Expression(Box<Expression>)
}

impl Value {
    pub fn value(self) -> f64 {
        match self {
            Value::Primitive(value) => value,
            Value::Expression(exp) => exp.eval()
        }
    }
}

pub struct Expression {
    pub a: Value,
    pub b: Value,
    pub action: Actions,
}

impl Expression {
    fn eval(self) -> f64 {
        let a = self.a.value();
        let b = self.b.value();
        match self.action {
            Actions::ADD => a + b,
            Actions::SUBTRACT => a - b,
            Actions::DIVIDE => a / b,
            Actions::MULTIPLY => a * b,
            Actions::POWER => a.powf(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_primitive_returns_value() {
        let val: f64 = 4.0;
        let value = Value::Primitive(val);
        assert_eq!(value.value(), val);
    }

    #[test]
    fn value_expression_returns_result() {
        let value = Value::Expression(Box::new(Expression {
            a: Value::Primitive(2.0),
            b: Value::Primitive(1.0),
            action: Actions::ADD
        }));
        assert_eq!(value.value(), 3.0);
    }

    #[test]
    fn expression_executes_addition() {
        let exp = Expression {
            a: Value::Primitive(1.0),
            b: Value::Primitive(2.0),
            action: Actions::ADD
        };
        assert_eq!(exp.eval(), 3.0);
    }

    #[test]
    fn expression_executes_subtraction() {
        let exp = Expression {
            a: Value::Primitive(1.0),
            b: Value::Primitive(2.0),
            action: Actions::SUBTRACT
        };
        assert_eq!(exp.eval(), -1.0);
    }

    #[test]
    fn expression_executes_multiplication() {
        let exp = Expression {
            a: Value::Primitive(1.0),
            b: Value::Primitive(2.0),
            action: Actions::MULTIPLY
        };
        assert_eq!(exp.eval(), 2.0);
    }

    #[test]
    fn expression_executes_division() {
        let exp = Expression {
            a: Value::Primitive(1.0),
            b: Value::Primitive(2.0),
            action: Actions::DIVIDE
        };
        assert_eq!(exp.eval(), 0.5);
    }

    #[test]
    fn expression_executes_exponentiation() {
        let exp = Expression {
            a: Value::Primitive(1.0),
            b: Value::Primitive(2.0),
            action: Actions::POWER
        };
        assert_eq!(exp.eval(), 1.0);
    }

    #[test]
    fn expression_executes_nested_addition() {
        let exp = Expression {
            a: Value::Primitive(1.0),
            b: Value::Expression(Box::new(Expression {
                a: Value::Primitive(2.0),
                b: Value::Primitive(3.0),
                action: Actions::ADD
            })),
            action: Actions::ADD
        };
        assert_eq!(exp.eval(), 6.0);
    }
}