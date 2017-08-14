use calc::*;

pub fn parse(input: String) -> Value {
    let mut a: Option<Value> = None;
    let mut b: Option<Value> = None;
    let mut action: Option<Actions> = None;
    let mut iter = input.trim().chars();
    let mut next = iter.next();
    while next.is_some() {
        match next.unwrap() {
            '(' | ')' => {
                let mut sub = String::new();
                next = iter.next();
                while next.is_some() && next.unwrap() != ')' {
                    sub.push(next.unwrap());
                    next = iter.next();
                }
                if next.is_none() {
                    panic!("Missing )");
                }
                if a.is_none() {
                    a = Some(parse(sub));
                }else if b.is_none() {
                    b = Some(parse(sub));
                }else {
                    print!("Whoops Brackets");
                }
            },
            '+' | '-' | '*' | '/' => {
                if action.is_some() && a.is_some() && b.is_some() {
                    a = Some(Value::Expression(Box::new(Expression {
                        a: a.unwrap(),
                        b: b.unwrap(),
                        action: action.unwrap()
                    })));
                    b = None;
                }
                action = match next.unwrap() {
                    '+' => Some(Actions::ADD),
                    '-' => Some(Actions::SUBTRACT),
                    '*' => Some(Actions::MULTIPLY),
                    '/' => Some(Actions::DIVIDE),
                    _ => None
                };
            },
            '^' => action = Some(Actions::POWER),
            '0'...'9' | '.' => {
                let mut current = String::new();
                while next.is_some() {
                    match next.unwrap() {
                        '0'...'9' | '.' => current.push(next.unwrap()),
                        _ => break,
                    };
                    next = iter.next();
                }
                let value = match current.parse() {
                    Ok(val) => Some(Value::Primitive(val)),
                    Err(_) => None
                };
                if a.is_none() {
                    a = value;
                }else if b.is_none() {
                    b = value;
                }else {
                    println!("Whoops Numbers");
                }
            },
            _ => {},
        }
        next = iter.next();
    }
    if a.is_some() && b.is_some() && action.is_some() {
        return Value::Expression(Box::new(Expression {
            a: a.unwrap(),
            b: b.unwrap(),
            action: action.unwrap()
        }));
    }else if a.is_some() && b.is_none() && action.is_none() {
        return a.unwrap();
    }
    panic!("Invalid Expression");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_return_primitive() {
        let input = String::from("1");
        assert_eq!(parse(input).value(), 1.0);
    }

    #[test]
    fn parse_should_add() {
        let input = String::from("1 + 1");
        assert_eq!(parse(input).value(), 2.0);
    }

    #[test]
    fn parse_should_subtract() {
        let input = String::from("2 - 1");
        assert_eq!(parse(input).value(), 1.0);
    }

    #[test]
    fn parse_should_multiply() {
        let input = String::from("2 * 1.5");
        assert_eq!(parse(input).value(), 3.0);
    }

    #[test]
    fn parse_should_divide() {
        let input = String::from("2.5 / 0.5");
        assert_eq!(parse(input).value(), 5.0);
    }

    #[test]
    fn parse_should_add_three() {
        let input = String::from("1 + 1 + 1");
        assert_eq!(parse(input).value(), 3.0);
    }
}