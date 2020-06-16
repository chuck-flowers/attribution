extern crate proc_macro;

use attribution::AttrArgs;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::FnArg;
use syn::ItemFn;
use syn::LitStr;
use syn::Pat;

#[derive(AttrArgs)]
enum EzTraceArgs {
    StartAndEnd { start: String, end: String },
    Start { start: String },
    End { end: String },
    Silent,
}

#[proc_macro_attribute]
pub fn ez_trace(attr: TokenStream, tagged: TokenStream) -> TokenStream {
    let attribute = parse_macro_input!(attr as EzTraceArgs);
    let mut function = parse_macro_input!(tagged as ItemFn);
    wrap_function(&mut function, &attribute);

    (quote! {
        #function
    })
    .into()
}

fn wrap_function(function: &mut ItemFn, attr: &EzTraceArgs) {
    fn make_template(msg: Option<&String>) -> Option<syn::LitStr> {
        msg.as_ref()
            .map(|template| syn::LitStr::new(template, Span::call_site()))
    }

    // Get the name of the function
    let name = LitStr::new(&function.sig.ident.to_string(), Span::call_site());

    // Build the comma separated list of arg names
    let args: Punctuated<_, syn::Token![,]> = function
        .sig
        .inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => quote! { &self },
            FnArg::Typed(pat_type) => match pat_type.pat.as_ref() {
                Pat::Ident(pat_ident) => {
                    let ident = &pat_ident.ident;
                    quote! { &#ident }
                }
                _ => panic!("Only simple parameter types are supported"),
            },
        })
        .collect();

    // Create the template that is used to format the args
    let mut arg_templates = if args.is_empty() {
        String::from("")
    } else {
        String::from("{}")
    };

    for _ in args.iter().skip(1) {
        arg_templates.push_str(", {}");
    }

    // Build the string builder which will be used at runtime
    let arg_templates = LitStr::new(&arg_templates, Span::call_site());
    let args = quote! {
        format!(#arg_templates, #args);
    };

    let start_param_val = match attr {
        EzTraceArgs::StartAndEnd { start, .. } => Some(start),
        EzTraceArgs::Start { start } => Some(start),
        _ => None,
    };

    let end_param_val = match attr {
        EzTraceArgs::StartAndEnd { end, .. } => Some(end),
        EzTraceArgs::End { end } => Some(end),
        _ => None,
    };

    // Create the start message if one has been defined
    let start_msg = make_template(start_param_val)
        .map(|template| {
            quote! {
                let args = #args;
                println!(#template, name = #name, args = args);
            }
        })
        .unwrap_or_default();

    // Create the end message if one has been defined
    let end_msg = make_template(end_param_val)
        .map(|template| {
            quote! {
                let args = #args;
                println!(#template, name = #name, args = args, result = result);
            }
        })
        .unwrap_or_default();

    // Wrap the old body with the messages if either are defined
    if !start_msg.is_empty() || !end_msg.is_empty() {
        let body = &function.block;
        let new_body: syn::Block = parse_quote! {
            {
                #start_msg
                let result = #body;
                #end_msg
                result
            }
        };

        function.block = Box::new(new_body);
    }
}
