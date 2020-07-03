use crate::ParamVal;
use core::convert::TryFrom;
use core::convert::TryInto;
use syn::parse::Error as ParseError;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::Expr;
use syn::ExprArray;
use syn::ExprLit;
use syn::ExprUnary;
use syn::Lit;
use syn::UnOp;

impl Parse for ParamVal {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        input.parse::<Expr>().and_then(TryInto::try_into)
    }
}

impl TryFrom<Expr> for ParamVal {
    type Error = ParseError;
    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        match value {
            Expr::Array(expr_array) => expr_array.try_into(),
            Expr::Lit(expr_lit) => expr_lit.try_into(),
            Expr::Unary(expr_unary) => expr_unary.try_into(),
            other_expr => Err(ParseError::new_spanned(
                other_expr,
                "Unsupported expression",
            )),
        }
    }
}

impl TryFrom<ExprArray> for ParamVal {
    type Error = ParseError;
    fn try_from(expr_array: ExprArray) -> Result<Self, Self::Error> {
        expr_array
            .elems
            .into_iter()
            .map(|expr| ParamVal::try_from(expr))
            .collect::<Result<Vec<ParamVal>, _>>()
            .map(|vals| ParamVal::Array(vals))
    }
}

impl TryFrom<ExprLit> for ParamVal {
    type Error = ParseError;
    fn try_from(ExprLit { lit, .. }: ExprLit) -> Result<Self, Self::Error> {
        lit.try_into()
    }
}

impl TryFrom<ExprUnary> for ParamVal {
    type Error = ParseError;
    fn try_from(ExprUnary { expr, op, .. }: ExprUnary) -> Result<Self, Self::Error> {
        match op {
            UnOp::Neg(_) => match *expr {
                Expr::Lit(ExprLit { lit, .. }) => match lit {
                    Lit::Int(i) => i.base10_parse::<i64>().map(|int| ParamVal::Int(-int)),
                    Lit::Float(f) => f.base10_parse::<f64>().map(|float| ParamVal::Float(-float)),
                    other_neg_lit => Err(ParseError::new_spanned(
                        other_neg_lit,
                        "Non-negatable literal",
                    )),
                },
                other_neg_expr => Err(ParseError::new_spanned(
                    other_neg_expr,
                    "Non-negatable expression",
                )),
            },
            other_op => Err(ParseError::new_spanned(
                other_op,
                "Unsupported parameter value operator",
            )),
        }
    }
}

impl TryFrom<Lit> for ParamVal {
    type Error = ParseError;
    fn try_from(lit: Lit) -> Result<Self, Self::Error> {
        match lit {
            Lit::Bool(b) => Ok(ParamVal::Bool(b.value)),
            Lit::Int(i) => i.base10_parse::<i64>().map(|int| ParamVal::Int(int)),
            Lit::Float(f) => f.base10_parse::<f64>().map(|float| ParamVal::Float(float)),
            Lit::Str(s) => Ok(ParamVal::Str(s.value())),
            other_lit => Err(ParseError::new_spanned(other_lit, "Unrecognized literal")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn parse_array() {
        let array_val: ParamVal = parse_quote!([1, 2, 3]);
        assert_eq!(
            array_val,
            ParamVal::Array(vec![ParamVal::Int(1), ParamVal::Int(2), ParamVal::Int(3)])
        )
    }

    #[test]
    fn parse_bool() {
        let bool_val: ParamVal = parse_quote!(true);
        assert_eq!(bool_val, ParamVal::Bool(true));
    }

    #[test]
    fn parse_unsigned_float() {
        let float_val: ParamVal = parse_quote!(1.0);
        assert_eq!(float_val, ParamVal::Float(1.0));
    }

    #[test]
    fn parse_signed_float() {
        let float_val: ParamVal = parse_quote!(-1.0);
        assert_eq!(float_val, ParamVal::Float(-1.0));
    }

    #[test]
    fn parse_unsigned_int() {
        let int_val: ParamVal = parse_quote!(1);
        assert_eq!(int_val, ParamVal::Int(1));
    }

    #[test]
    fn parse_signed_int() {
        let int_val: ParamVal = parse_quote!(-1);
        assert_eq!(int_val, ParamVal::Int(-1));
    }

    #[test]
    fn parse_string() {
        let str_val: ParamVal = parse_quote!("foo");
        assert_eq!(str_val, "foo".into())
    }
}
