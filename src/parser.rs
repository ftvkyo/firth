use nom::combinator::all_consuming;

#[derive(PartialEq, Clone, Debug)]
pub enum Word<'a> {
    Int(i32),
    Ident(&'a str),
    Op(&'a str),
}

pub type Expression<'a> = Vec<Word<'a>>;

mod parser {
    use super::*;

    use nom::{
        branch::alt,
        bytes::complete::is_a,
        character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
        combinator::{map_res, opt, recognize},
        multi::{many0, many1, separated_list1},
        sequence::{delimited, pair, tuple},
        IResult,
    };

    pub fn integer(input: &str) -> IResult<&str, Word> {
        map_res(recognize(pair(opt(char('-')), digit1)), |res| {
            str::parse(res).map(|res| Word::Int(res))
        })(input)
    }

    pub fn identifier(input: &str) -> IResult<&str, Word> {
        recognize(tuple((
            alpha1,
            many0(alt((alphanumeric1, is_a("-_")))),
            opt(char('?')),
        )))(input)
        .map(|(rest, ident)| (rest, Word::Ident(ident)))
    }

    pub fn operator(input: &str) -> IResult<&str, Word> {
        recognize(many1(is_a("-+/*=|&<>")))(input).map(|(rest, op)| (rest, Word::Op(op)))
    }

    pub fn word(input: &str) -> IResult<&str, Word> {
        alt((integer, identifier, operator))(input)
    }

    pub fn expression(input: &str) -> IResult<&str, Vec<Word>> {
        delimited(multispace0, separated_list1(multispace1, word), multispace0)(input)
    }
}

pub fn parse_expression(input: &str) -> Result<Expression, ()> {
    all_consuming(parser::expression)(input)
        .map(|(_, v)| v)
        .map_err(|_| ())
}

#[cfg(test)]
mod test {

    use nom::combinator::all_consuming;

    use super::parser::*;
    use super::*;

    use Word::*;

    #[test]
    fn integers_good() {
        let integer = |input| all_consuming(integer)(input);
        let ok = |input| Ok(("", Int(input)));

        assert_eq!(integer("1"), ok(1));
        assert_eq!(integer("1"), ok(1));
        assert_eq!(integer("0"), ok(0));
        assert_eq!(integer("-1"), ok(-1));
        assert_eq!(integer("255"), ok(255));
        assert_eq!(integer("0001"), ok(1));
    }

    #[test]
    fn integers_bad() {
        let integer = |input| all_consuming(integer)(input);

        assert!(matches!(integer("1-"), Err(_)));
        assert!(matches!(integer("_1"), Err(_)));
        assert!(matches!(integer("NaN"), Err(_)));

        // TODO: Should be a parsing error
        assert!(matches!(integer("100000000000000000000"), Err(_)));
    }

    #[test]
    fn identifiers_good() {
        let identifier = |input| all_consuming(identifier)(input);
        let ok = |input| Ok(("", Ident(input)));

        assert_eq!(identifier("abc"), ok("abc"));
        assert_eq!(identifier("ident-123"), ok("ident-123"));
        assert_eq!(identifier("is_potato?"), ok("is_potato?"));
    }

    #[test]
    fn identifiers_bad() {
        let identifier = |input| all_consuming(identifier)(input);

        // Can't start with a number
        assert!(matches!(identifier("1"), Err(_)));
        assert!(matches!(identifier("1abc"), Err(_)));
    }

    #[test]
    fn operators_good() {
        let operator = |input| all_consuming(operator)(input);
        let ok = |input| Ok(("", Op(input)));

        assert_eq!(operator("+"), ok("+"));
        assert_eq!(operator("/-/"), ok("/-/"));
    }

    #[test]
    fn operators_bad() {
        let operator = |input| all_consuming(operator)(input);

        assert!(matches!(operator("?"), Err(_)));
    }

    #[test]
    fn expressions_good() {
        let expression = |input| all_consuming(expression)(input);
        let ok = |input: &'static [Word<'_>]| Ok(("", Vec::from(input)));

        assert_eq!(expression("1 2 +"), ok(&[Int(1), Int(2), Op("+")]));

        assert_eq!(expression("1"), ok(&[Int(1)]));
        assert_eq!(expression("1 "), ok(&[Int(1)]));
        assert_eq!(expression(" 1"), ok(&[Int(1)]));
        assert_eq!(expression(" 1 "), ok(&[Int(1)]));
    }

    #[test]
    fn expressions_bad() {
        let expression = |input| all_consuming(expression)(input);

        assert!(matches!(expression("1-"), Err(_)));
    }
}
