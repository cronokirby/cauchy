use nom::types::CompleteStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Scalar(f32),
    Var,
    I,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Exp(Box<Expr>),
    Ln(Box<Expr>)
}

pub enum Todo {
    Op(i32),
    Expr(Expr)
}


named!(scalar<CompleteStr, Expr>,
    map!(nom::float, Expr::Scalar)
);

named!(the_i<CompleteStr, Expr>,
    map!(tag!("i"), |_| Expr::I)
);

named!(var<CompleteStr, Expr>,
    map!(tag!("z"), |_| Expr::Var)
);

named!(sin<CompleteStr, Expr>,
    map!(delimited!(tag!("sin("), expr, tag!(")")),
        |x| Expr::Sin(Box::new(x))
    )
);

named!(exp<CompleteStr, Expr>,
    map!(delimited!(tag!("exp("), expr, tag!(")")),
        |x| Expr::Exp(Box::new(x))
    )
);

named!(ln<CompleteStr, Expr>,
    map!(delimited!(tag!("ln("), expr, tag!(")")),
        |x| Expr::Ln(Box::new(x))
    )
);

named!(factor<CompleteStr, Expr>,
    alt!(
        ws!(scalar) | 
        ws!(the_i)  |
        ws!(var)    |
        ws!(sin)    |
        ws!(exp)    |
        ws!(ln)     |
        ws!(delimited!(tag!("("), expr, tag!(")")))
    )
);

named!(expr1<CompleteStr, Expr>, do_parse!(
    init: factor >>
    res: fold_many0!(
        tuple!(
            alt!(tag!("*") | tag!("/")),
            factor
        ),
        init,
        |acc, v: (_, Expr)| {
            if v.0 == "*".into() {
                Expr::Mul(Box::new(acc), Box::new(v.1))
            } else {
                Expr::Div(Box::new(acc), Box::new(v.1))
            }
        }
    )
    >> (res)
));

named!(pub expr<CompleteStr, Expr>, do_parse!(
    init: expr1 >>
    res: fold_many0!(
        tuple!(
            alt!(tag!("+") | tag!("-")),
            expr1
        ),
        init,
        |acc, v: (_, Expr)| {
            if v.0 == "+".into() {
                Expr::Add(Box::new(acc), Box::new(v.1))
            } else {
                Expr::Sub(Box::new(acc), Box::new(v.1))
            }
        }
    )
    >> (res)
));


pub fn make_rpn(input: &str, tokens: &mut [i32], floats: &mut [f32]) -> bool {
    for x in tokens.iter_mut() {
        *x = 0;
    }
    for x in floats.iter_mut() {
        *x = 0.0;
    }
    let mut t_i: usize = 0; 
    let mut f_i: usize = 0;
    let mut todos: Vec<Todo> = Vec::new();
    if let Ok((_, expr)) = expr(input.into()) {
        todos.push(Todo::Expr(expr));
    } else {
        return false;
    }
    while let Some(todo) = todos.pop() {
        if t_i >= tokens.len() || f_i >= floats.len() {
            return false;
        }
        use self::Expr::*;
        match todo {
            Todo::Expr(Scalar(f)) => {
                floats[f_i] = f;
                tokens[t_i] = !(f_i as i32);
                f_i += 1;
                t_i += 1;
            }
            Todo::Expr(I) => {
                tokens[t_i] = 2;
                t_i += 1;
            }
            Todo::Expr(Var) => {
                tokens[t_i] = 1;
                t_i += 1;
            }
            Todo::Expr(Add(e1, e2)) => {
                todos.push(Todo::Op(3));
                todos.push(Todo::Expr(*e2));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Expr(Sub(e1, e2)) => {
                todos.push(Todo::Op(4));
                todos.push(Todo::Expr(*e2));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Expr(Mul(e1, e2)) => {
                todos.push(Todo::Op(5));
                todos.push(Todo::Expr(*e2));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Expr(Div(e1, e2)) => {
                todos.push(Todo::Op(6));
                todos.push(Todo::Expr(*e2));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Expr(Sin(e1)) => {
                todos.push(Todo::Op(7));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Expr(Exp(e1)) => {
                todos.push(Todo::Op(8));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Expr(Ln(e1)) => {
                todos.push(Todo::Op(9));
                todos.push(Todo::Expr(*e1));
            }
            Todo::Op(o) => {
                tokens[t_i] = o;
                t_i += 1;
            }
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar() {
        let res = scalar("2.125".into());
        println!("{:?}", res);
        assert!(res.is_ok());
        let (_, s) = res.unwrap();
        assert_eq!(s, Expr::Scalar(2.125));
    }

    #[test]
    fn test_i() {
        let res = the_i("i".into());
        assert!(res.is_ok());
        let (_, s) = res.unwrap();
        assert_eq!(s, Expr::I);
    }

    #[test]
    fn test_var() {
        let res = var("z".into());
        assert!(res.is_ok());
        let (_, s) = res.unwrap();
        assert_eq!(s, Expr::Var);
    }

    #[test]
    fn test_expr() {
        let res1 = expr("1.3 + 4.0".into()); 
        assert!(res1.is_ok());
        let (_, s1) = res1.unwrap();
        assert_eq!(s1, Expr::Add(Box::new(Expr::Scalar(1.3)), Box::new(Expr::Scalar(4.0))));
        let res2 = expr("1.3 - 4.0".into());
        assert!(res2.is_ok());
        let (_, s2) = res2.unwrap();
        assert_eq!(s2, Expr::Sub(Box::new(Expr::Scalar(1.3)), Box::new(Expr::Scalar(4.0))));
        let res3 = expr("1.3 * 4.0".into());
        assert!(res3.is_ok());
        let (_, s3) = res3.unwrap();
        assert_eq!(s3, Expr::Mul(Box::new(Expr::Scalar(1.3)), Box::new(Expr::Scalar(4.0))));
        let res4 = expr("1.3 + 4.0 * i".into());
        assert!(res4.is_ok());
        let (_, s4) = res4.unwrap();
        assert_eq!(s4,
            Expr::Add(
                Box::new(Expr::Scalar(1.3)),
                Box::new(Expr::Mul(
                    Box::new(Expr::Scalar(4.0)),
                    Box::new(Expr::I)
                )
            )
        ));
    }

}