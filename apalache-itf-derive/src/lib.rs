extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, FieldsUnnamed,
    GenericParam, Generics,
};

#[proc_macro_derive(DecodeItfValue)]
pub fn derive_decode_itf_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let decode = itf_decode(&input.data);

    let expanded = quote! {
        impl #impl_generics ::apalache_itf::DecodeItfValue for #name #ty_generics
            #where_clause {

            fn decode(value: ::apalache_itf::Value) -> Result<Self, ::apalache_itf::DecodeError> {
                #decode
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TryFromRawState)]
pub fn derive_try_from_raw_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let try_from = try_from_raw_state(&input.data);

    let expanded = quote! {
        impl #impl_generics TryFrom<::apalache_itf::raw::State> for #name #ty_generics
            #where_clause {

            type Error = ::apalache_itf::DecodeError;

            fn try_from(mut raw_state: ::apalache_itf::raw::State) -> Result<Self, Self::Error> {
                #try_from
            }
        }
    };

    TokenStream::from(expanded)
}

/// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::apalache_itf::DecodeItfValue));
        }
    }
    generics
}

fn itf_decode(data: &Data) -> TokenStream2 {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap();
                    let quote_name = name.to_string();
                    let ty = &f.ty;

                    quote_spanned! { f.span() =>
                        #name : <#ty as ::apalache_itf::DecodeItfValue>::decode(
                            map
                                .remove(#quote_name)
                                .ok_or(::apalache_itf::DecodeError::FieldNotFound(#quote_name))?
                        )?
                    }
                });

                quote! {
                    let mut map = <HashMap<String, Value> as DecodeItfValue>::decode(value)?;

                    Ok(Self {
                        #(#recurse ,)*
                    })
                }
            }
            Fields::Unnamed(ref fields) => {
                let types = fields_to_tuple_type(fields);

                quote! {
                    Ok(<#types as ::apalache_itf::DecodeItfValue>::decode(value))
                }
            }
            Fields::Unit => {
                quote!(Self)
            }
        },

        Data::Enum(ref data) => {
            let variants = data.variants.iter().map(|v| {
                assert!(matches!(v.fields, Fields::Unit));
                let name = &v.ident;
                let quote_name = name.to_string();
                quote_spanned! { v.span() =>
                    Value::String(s) if s == #quote_name => Ok(Self::#name)
                }
            });

            quote! {
                use ::apalache_itf::Value;

                match value {
                    #(#variants, )*
                    _ => Err(DecodeError::InvalidType("string"))
                }
            }
        }

        Data::Union(_) => unimplemented!(),
    }
}

fn try_from_raw_state(data: &Data) -> TokenStream2 {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap();
                    let quote_name = name.to_string();
                    let ty = &f.ty;

                    quote_spanned! { f.span() =>
                        #name : <#ty as DecodeItfValue>::decode(
                            raw_state
                                .values
                                .remove(#quote_name)
                                .ok_or(DecodeError::FieldNotFound(#quote_name))?
                        )?
                    }
                });

                quote! {
                    use ::std::collections::HashMap;
                    use ::apalache_itf::{Value, DecodeItfValue, DecodeError};

                    Ok(Self {
                        #(#recurse ,)*
                    })
                }
            }

            Fields::Unnamed(ref fields) => {
                let types = fields_to_tuple_type(fields);

                quote! {
                    use ::apalache_itf::DecodeItfValue;

                    Ok(<#types as DecodeItfValue>::decode(value))
                }
            }

            Fields::Unit => {
                quote!(Self)
            }
        },

        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn fields_to_tuple_type(fields: &FieldsUnnamed) -> TokenStream2 {
    let types = fields.unnamed.iter().map(|f| &f.ty);

    quote! {
        (#(#types ,)*)
    }
}
