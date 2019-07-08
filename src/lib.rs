#![warn(clippy::all)]

extern crate proc_macro;

mod field_spec;
mod attr_map;

use std::convert::TryFrom;
use field_spec::FieldSpec;
use attr_map::AttrMap;
use attr_map::AttrVal;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn attr_args(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the inputs
    let input_attr = syn::parse_macro_input!(attr as AttrMap);
    let input_struct = if let syn::Item::Struct(struct_data) = syn::parse_macro_input!(input as syn::Item) {
        struct_data
    } else {
        panic!("The attribute can only be applied to structs")
    };

    let fields = extract_fields(&input_struct);
    let output = impl_parse(&input_attr, &fields);

    (quote! {
        #input_struct
    })
    .into()
}

fn extract_fields(input_struct: &syn::ItemStruct) -> Vec<FieldSpec> {
    if let syn::Fields::Named(fields) = &input_struct.fields {
        fields.named.iter().map(TryFrom::try_from).map(Result::unwrap).collect()
    } else {
        panic!("The attribute can only be applied to structs with named fields")
    }
}

fn impl_parse(input_attr: &AttrMap, fields: &Vec<FieldSpec>) -> proc_macro2::TokenStream {
    let idents = fields.iter().map(|f| f.ident());
    let tys = fields.iter().map(|f| f.ty());

    let inital_field_declarations = quote! {
        #(let mut #idents : Option<#tys> = None;)*
    };

    quote! {
        #inital_field_declarations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn impl_parse_test() {
        let mut attr_map = AttrMap::new();
        attr_map.insert("foo".into(), AttrVal::Str("fooVal".into()));
        attr_map.insert("bar".into(), AttrVal::Integer(1));
        attr_map.insert("baz".into(), AttrVal::Bool(true));

        let foo_ident = parse_quote!(foo);
        let bar_ident = parse_quote!(bar);
        let baz_ident = parse_quote!(baz);
        let foo_type = parse_quote!(String);
        let bar_type = parse_quote!(u32);
        let baz_type = parse_quote!(bool);

        let fields = vec![
            FieldSpec::new(&foo_ident, &foo_type),
            FieldSpec::new(&bar_ident, &bar_type),
            FieldSpec::new(&baz_ident, &baz_type)
        ];

        let output = impl_parse(&attr_map, &fields);
        assert_eq!(
            output.to_string(),
            (quote! {
                let mut foo: Option<String> = None;
                let mut bar: Option<u32> = None;
                let mut baz: Option<bool> = None;
            }).to_string()
        )
    }
}