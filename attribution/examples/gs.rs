extern crate proc_macro;

use attribution::attr_args;
use attribution::DynamicParameters;
use attribution::ParamVal;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Field;
use syn::Fields;
use syn::Ident;
use syn::Item;
use syn::ItemStruct;

#[attr_args]
struct GetterSetterArgs {
    field_names: DynamicParameters,
}

#[proc_macro_attribute]
pub fn gs(attr: TokenStream, tagged: TokenStream) -> TokenStream {
    impl_gs(attr, tagged).into()
}

fn impl_gs(
    into_attr: impl Into<TokenStream2>,
    into_tagged: impl Into<TokenStream2>,
) -> TokenStream2 {
    let attr = into_attr.into();
    let tagged = into_tagged.into();
    let args: GetterSetterArgs = syn::parse2(attr).expect("Unabled to parse attribute");

    if let Ok(Item::Struct(tagged_struct)) = syn::parse2(tagged) {
        add_methods(tagged_struct, args.field_names)
    } else {
        panic!("The attribute can only be applied to structs")
    }
}

fn add_methods(tagged_struct: ItemStruct, parameters: DynamicParameters) -> TokenStream2 {
    if let Fields::Named(fields) = &tagged_struct.fields {
        // Gets all the methods for the identifiers that were marked as true
        let methods = fields
            .named
            .iter()
            .filter(|field| {
                let ident_text: String = field.ident.as_ref().unwrap().to_string();

                *parameters
                    .get(&ident_text)
                    .map(|param_val| {
                        if let ParamVal::Bool(b) = param_val {
                            b
                        } else {
                            &false
                        }
                    })
                    .unwrap_or(&false)
            })
            .map(make_method);

        let impl_block_name = tagged_struct.ident.clone();
        quote! {
            #tagged_struct

            impl #impl_block_name {
                #(#methods)*
            }
        }
    } else {
        panic!("The tagged struct must have named fields")
    }
}

fn make_method(field: &Field) -> TokenStream2 {
    let field_ident = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let setter_field = field_ident.clone();
    let setter_type = field_type.clone();
    let setter_name = Ident::new(
        &format!("set_{}", field_ident.to_string()),
        Span::call_site(),
    );

    let getter_field = field_ident.clone();
    let getter_type = syn::TypeReference {
        and_token: <syn::Token![&]>::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(field_type.clone()),
    };
    let getter_name = Ident::new(
        &format!("get_{}", field_ident.to_string()),
        Span::call_site(),
    );

    quote! {
        fn #getter_name(&self) -> #getter_type {
            self.#getter_field
        }

        fn #setter_name(&mut self, new_val: #setter_type) {
            self.#setter_field = new_val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    #[test]
    fn usage_test() {
        let raw_attr: syn::Attribute = syn::parse_quote! {
            #[gs(foo = true)]
        };

        let raw_struct: Item = syn::parse_quote! {
            struct MyStruct {
                foo: u32,
                bar: bool,
                baz: String
            }
        };

        let expected = quote! {
            struct MyStruct {
                foo: u32,
                bar: bool,
                baz: String
            }

            impl MyStruct {
                fn get_foo(&self) -> &u32 {
                    self.foo
                }

                fn set_foo(&mut self, new_val: u32) {
                    self.foo = new_val;
                }
            }
        };

        if let proc_macro2::TokenTree::Group(group) = raw_attr.tts.into_iter().next().unwrap() {
            let actual = impl_gs(group.stream(), raw_struct.into_token_stream());
            assert_eq!(actual.to_string(), expected.to_string());
        } else {
            panic!()
        }
    }
}
