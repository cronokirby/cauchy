use nom::types::CompleteStr;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Scalar(f32),
    I,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>)
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

named!(expr<CompleteStr, Expr>, do_parse!(
    init: factor >>
    res: fold_many0!(
        tuple!(
            alt!(tag!("+") | tag!("-")),
            factor
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
    }
}