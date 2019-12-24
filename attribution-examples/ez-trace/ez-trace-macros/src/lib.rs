extern crate proc_macro;

use attribution::attr_args;
use proc_macro::TokenStream;
use syn::ItemFn;
use syn::parse_macro_input;
use quote::quote;
use syn::parse_quote;

#[attr_args]
struct EzTraceArgs {}

#[proc_macro_attribute]
pub fn ez_trace(attr: TokenStream, tagged: TokenStream) -> TokenStream {
    let attribute = parse_macro_input!(attr as EzTraceArgs);
    let mut function = parse_macro_input!(tagged as ItemFn);
    wrap_function(&mut function, &attribute);

    (quote! {
        #function
    }).into()
}

fn wrap_function(function: &mut ItemFn, _: &EzTraceArgs) {
    let function_name = function.sig.ident.to_string();
    let function_name_lit = syn::LitStr::new(&function_name, proc_macro2::Span::call_site());
    let body = &function.block;
    let new_body: syn::Block = parse_quote! {
        {
            let result = #body;
            println!("{}: {}", #function_name_lit, result);
            result
        }
    };

    function.block = Box::new(new_body);
}