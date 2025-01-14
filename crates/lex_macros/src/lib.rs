//! Derive proc-macro  definitions for the `pluine_lex::Spanned` in  trait.
//! Only to be used within `pluine_lex` itself.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident};

/// Derives `Spanned`
///
/// ### Supported
/// - Enum whose variants contain a single unnamed field.
/// - Tuple and named structs, span field needs to be marked with a `#[span]` attribute
///
/// ### Not supported
/// - Enum with unit variants
/// - Struct unions
///
/// ### Can be supported if needed
/// - Enum variants with multiple unnamed fields
/// - Enum variants with named fields
#[proc_macro_derive(Spanned, attributes(span))]
pub fn derive_mock(token_stream: TokenStream) -> TokenStream {
    derive_mock_impl(token_stream)
}

fn derive_mock_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let identifier = type_definition.ident;

    let self_definition_result = match type_definition.data {
        Data::Struct(data_struct) => derive_struct(data_struct),
        Data::Enum(data_enum) => derive_enum(&identifier, data_enum),
        Data::Union(data_union) => Err(syn::Error::new(data_union.union_token.span, "union types not supported")),
    };

    let (impl_generics, type_generics, where_clause) = type_definition.generics.split_for_impl();

    match self_definition_result {
        Ok(self_definition) => {
            quote! {
                impl #impl_generics crate::Spanned for #identifier #type_generics #where_clause {
                    fn span(&self) -> crate::Span {
                        #self_definition
                    }
                }

                impl #impl_generics crate::private::Sealed for #identifier #type_generics #where_clause {}
            }
        }
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn derive_struct(data_struct: syn::DataStruct) -> syn::Result<proc_macro2::TokenStream> {
    return match data_struct.fields {
        Fields::Named(ref fields_named) => {
            let (_, span_field) = find_span_field(&data_struct, fields_named.named.iter())?;
            let span_field_ident = &span_field.ident;

            Ok(quote! { self.#span_field_ident })
        }
        Fields::Unnamed(ref fields_unnamed) => {
            let span_tuple_struct_index = match fields_unnamed.unnamed.len() {
                0 => {
                    return Err(syn::Error::new(
                        data_struct.fields.span(),
                        "no tuple fields supplied, at least one required",
                    ));
                }
                1 => 0,
                _ => {
                    let (span_field_index, _) = find_span_field(&data_struct, fields_unnamed.unnamed.iter())?;
                    span_field_index
                }
            };

            let span_tuple_struct_index = syn::Index::from(span_tuple_struct_index);

            Ok(quote! { self.#span_tuple_struct_index })
        }
        Fields::Unit => Err(syn::Error::new(
            data_struct.struct_token.span,
            "unit structs are not supported as they contain no span",
        )),
    };

    fn find_span_field<'a>(
        data_struct: &'a syn::DataStruct,
        field_iter: impl Iterator<Item = &'a syn::Field>,
    ) -> syn::Result<(usize, &'a syn::Field)> {
        const SPAN_ATTRIBUTE_NAME: &str = "span";

        let mut span_field_iter = field_iter.enumerate().filter_map(|(index, field)| {
            field
                .attrs
                .clone()
                .iter()
                .find(|attribute| match &attribute.meta {
                    syn::Meta::Path(path) => path.is_ident(SPAN_ATTRIBUTE_NAME),
                    _ => false,
                })
                .map(|_| (index, field))
        });

        let Some(span_field) = span_field_iter.next() else {
            return Err(syn::Error::new(data_struct.fields.span(), "missing field annotated with #[span]"));
        };

        if let Some(_another_span_field) = span_field_iter.next() {
            return Err(syn::Error::new(
                data_struct.fields.span(),
                "expected only one #[span] field attribute, unable to infer which one to use.",
            ));
        }

        Ok(span_field)
    }
}

fn derive_enum(ident: &Ident, data_enum: syn::DataEnum) -> syn::Result<proc_macro2::TokenStream> {
    let mut variants_buffer = Vec::<proc_macro2::TokenStream>::with_capacity(data_enum.variants.len());

    for variant in data_enum.variants {
        match variant.fields {
            Fields::Named(_) => {
                return Err(syn::Error::new(
                    variant.span(),
                    "deriving `Spanned` on enums with named fields is currently not supported",
                ));
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut unnamed_fields_iter = fields_unnamed.unnamed.iter();

                if unnamed_fields_iter.next().is_none() {
                    return Err(syn::Error::new(
                        fields_unnamed.span(),
                        "no unnamed field found in enum variant which may implement `Span`",
                    ));
                };

                if let Some(another_span_field) = unnamed_fields_iter.next() {
                    return Err(syn::Error::new(
                        another_span_field.span(),
                        "deriving `Spanned` on enums variants with multiple unnamed fields is currently not supported",
                    ));
                }

                let variant_ident = variant.ident;

                variants_buffer.push(quote! { #ident::#variant_ident(inner) => inner.span() });
            }
            Fields::Unit => {
                return Err(syn::Error::new(
                    variant.span(),
                    "unit structs are not supported as they contain no span",
                ));
            }
        }
    }

    Ok(quote! {
        match self {
            #(#variants_buffer),*
        }
    })
}
