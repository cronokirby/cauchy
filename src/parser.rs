use nom::types::CompleteStr;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Scalar(f32),
    I,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}



named!(scalar<CompleteStr, Expr>,
    map!(nom::float, Expr::Scalar)
);

named!(the_i<CompleteStr, Expr>,
    map!(tag!("i"), |_| Expr::I)
);

named!(factor<CompleteStr, Expr>,
    alt!(
        ws!(scalar) | 
        ws!(the_i)  |
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

named!(expr<CompleteStr, Expr>, do_parse!(
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