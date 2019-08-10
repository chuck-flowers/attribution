extern crate proc_macro;

use attribution::attr_args;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;

#[attr_args]
struct EzTraceArgs {
    start: Option<String>,
    end: Option<String>,
}

#[proc_macro_attribute]
pub fn ez_trace(attr: TokenStream, tagged: TokenStream) -> TokenStream {
    ez_trace_impl(attr.into(), tagged.into()).into()
}

fn ez_trace_impl(
    attr: proc_macro2::TokenStream,
    tagged: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let args = syn::parse2(attr).unwrap();
    if let syn::Item::Fn(tagged_fn) = syn::parse2(tagged).unwrap() {
        impl_trace(args, tagged_fn)
    } else {
        panic!("The ez_trace attribute can only be applied to functions");
    }
}

fn impl_trace(attr: EzTraceArgs, mut tagged_fn: syn::ItemFn) -> proc_macro2::TokenStream {
    let start_statement = if let Some(start_text) = attr.start {
        let start_lit = syn::LitStr::new(&start_text, proc_macro2::Span::call_site());
        quote! {
            println!("{}", #start_lit);
        }
    } else {
        quote! {}
    };

    let end_statement = if let Some(end_text) = attr.end {
        let end_lit = syn::LitStr::new(&end_text, proc_macro2::Span::call_site());
        quote! {
            println!("{}", #end_lit);
        }
    } else {
        quote! {}
    };

    let old_block = tagged_fn.block;
    let new_block: syn::Block = syn::parse_quote! {
        {
            #start_statement
            let ret = #old_block;
            #end_statement
            ret
         }
    };

    tagged_fn.block = Box::new(new_block);

    tagged_fn.into_token_stream().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenTree;
    use std::iter::IntoIterator;
    use syn::parse_quote;
    use syn::Attribute;
    use syn::ItemFn;

    fn build_expected() -> ItemFn {
        parse_quote! {
            fn start() {
                println!("{}", "Starting...");
                let ret = { println!("start"); };
                ret
            }
        }
    }

    fn build_input_attr() -> Attribute {
        parse_quote! { #[ez_trace(start = "Starting...")] }
    }

    fn build_input_fn() -> ItemFn {
        parse_quote! {
            fn start() {
                println!("start");
            }
        }
    }

    #[test]
    fn example_trace_test() {
        let expected = build_expected();
        let expected_str = expected.into_token_stream().to_string();
        let raw_attr_start = build_input_attr();
        let raw_fn: ItemFn = build_input_fn();
        let attr_start_tts = raw_attr_start.tts;
        if let TokenTree::Group(group) = attr_start_tts.into_iter().next().unwrap() {
            let output = ez_trace_impl(group.stream(), raw_fn.into_token_stream());
            let output_str = output.into_token_stream().to_string();

            assert_eq!(output_str, expected_str);
        }
    }
}
