use attribution::attr_args;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Expr;
use syn::ItemFn;
use syn::LitBool;
use syn::LitFloat;
use syn::LitInt;
use syn::LitStr;
use syn::Stmt;

#[attr_args]
struct AttributeArgs {
    flag: bool,
    string: String,
    integer: i64,
    float: f64,
    array_of_integers: Vec<i64>,
}

#[proc_macro_attribute]
pub fn exhaustive(attr: TokenStream, tagged: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let mut item_fn = parse_macro_input!(tagged as ItemFn);
    for stmt in make_print_lines(attr) {
        item_fn.block.stmts.push(stmt)
    }

    item_fn.to_token_stream().into()
}

fn make_print_lines(attr: AttributeArgs) -> impl IntoIterator<Item = Stmt> {
    let flag_val = LitBool {
        value: attr.flag,
        span: proc_macro2::Span::call_site(),
    };
    let string_val = LitStr::new(&attr.string, Span::call_site());
    let integer_val = LitInt::new(&attr.integer.to_string(), Span::call_site());
    let float_val = LitFloat::new(&attr.float.to_string(), Span::call_site());
    let array_of_integers = attr
        .array_of_integers
        .into_iter()
        .map(|i| i.to_string())
        .map(|i| LitInt::new(&i, Span::call_site()));
    let array_of_integers_val: Expr = parse_quote! { vec![#(#array_of_integers),*] };
    vec![
        parse_quote! {
            println!("flag = {}", #flag_val);
        },
        parse_quote! {
            println!("string = {}", #string_val);
        },
        parse_quote! {
            println!("integer = {}", #integer_val);
        },
        parse_quote! {
            println!("float = {}", #float_val);
        },
        parse_quote! {
            println!("array_of_integers = {:?}", #array_of_integers_val);
        },
    ]
}
