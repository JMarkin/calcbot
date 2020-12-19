use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::multi::fold_many0;
use nom::number::complete::double;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use nom::{branch::alt, bytes::complete::tag};
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn factor(s: &str) -> IResult<&str, f64> {
    alt((ws(double), delimited(ws(tag("(")), ws(expr), ws(tag(")")))))(s)
}

fn term(s: &str) -> IResult<&str, f64> {
    let (i, res) = factor(s)?;
    fold_many0(
        tuple((alt((tag("*"), tag("/"))), factor)),
        res,
        |acc, v: (_, f64)| {
            if v.0 == "*" {
                acc * v.1
            } else {
                acc / v.1
            }
        },
    )(i)
}

fn expr(s: &str) -> IResult<&str, f64> {
    let (i, res) = term(s)?;
    fold_many0(
        tuple((alt((tag("+"), tag("-"))), term)),
        res,
        |acc, v: (_, f64)| {
            if v.0 == "+" {
                acc + v.1
            } else {
                acc - v.1
            }
        },
    )(i)
}

pub fn calc(s: &str) -> String {
    let resp = expr(s);
    let err = "Invalid, please try";
    match resp {
        Err(e) => err.to_string(),
        Ok((i, resp)) => {
            if i != "" {
                return err.to_string();
            }
            resp.to_string()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_factor() {
        assert_eq!(factor("1"), Ok(("", 1.0)));
        assert_eq!(factor("   1  "), Ok(("", 1.0)));
        assert_eq!(factor("1.0"), Ok(("", 1.0)));
    }

    #[test]
    fn test_term() {
        assert_eq!(term("2.0 * 3.0"), Ok(("", 6.0)));
        assert_eq!(term("2 * 3.0"), Ok(("", 6.0)));
        assert_eq!(term("3*2 / 3.0"), Ok(("", 2.0)));
    }

    #[test]
    fn test_expr() {
        assert_eq!(expr("2.0 * 3.0 +5"), Ok(("", 11.0)));
        assert_eq!(expr("0+2 * 3.0"), Ok(("", 6.0)));
        assert_eq!(expr("3*2 / 3.0 - 1"), Ok(("", 1.0)));
        assert_eq!(expr("3*2 / (3.0 - 1)"), Ok(("", 3.0)));
        assert_eq!(expr("(1+3)*2 / 4.0 - 1"), Ok(("", 1.0)));
    }
}
