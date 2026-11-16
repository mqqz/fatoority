//! Proc-macro helpers for validating invoice inputs used by `fatoora-core`.
//!
//! # Examples
//! ```rust
//! use fatoora_derive::Validate;
//!
//! #[derive(Validate)]
//! struct Payload {
//!     #[validate(non_empty)]
//!     name: String,
//! }
//! ```
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Attribute, Data, DeriveInput, Fields, Type, parse_macro_input};

mod rules;

fn extract_error_type(attrs: &[Attribute]) -> syn::Result<TokenStream2> {
    if let Some(attr) = attrs.iter().find(|a| a.path().is_ident("validate_error")) {
        let ty: Type = attr.parse_args()?;
        return Ok(ty.to_token_stream());
    }
    Ok(quote! { String })
}

fn extract_rules(attrs: &[Attribute]) -> syn::Result<Vec<String>> {
    let mut out = vec![];
    for attr in attrs.iter().filter(|a| a.path().is_ident("validate")) {
        attr.parse_nested_meta(|meta| {
            if let Some(id) = meta.path.get_ident() {
                out.push(id.to_string());
            } else {
                return Err(meta.error("validation rule must be a single identifier"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

/// Only allow rules on String for now.
fn is_string_type(ty: &Type) -> bool {
    match ty {
        Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| s.ident == "String")
            .unwrap_or(false),
        Type::Reference(r) => {
            if let Type::Path(p) = &*r.elem {
                p.path
                    .segments
                    .last()
                    .map(|s| s.ident == "String")
                    .unwrap_or(false)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Derive constructor-based validation for structs.
///
/// # Errors
/// Emits a compile error if rules are unknown or applied to unsupported field types.
#[proc_macro_derive(Validate, attributes(validate, validate_error))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    expand_validate(ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_validate(ast: DeriveInput) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let struct_name = ast.ident;
    let error_type = extract_error_type(&ast.attrs)?;
    let struct_rules = extract_rules(&ast.attrs)?;

    let mut ctor_params = vec![];
    let mut ctor_assigns = vec![];
    let mut validations = vec![];

    let fields = match ast.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(n) => n.named,
            _ => return Ok(quote! { compile_error!("Validate supports named structs only"); }),
        },
        _ => return Ok(quote! { compile_error!("Validate can only be used on structs"); }),
    };

    for field in fields {
        let ident = field.ident.unwrap();
        let ty = field.ty;

        ctor_params.push(quote! { #ident: #ty });
        ctor_assigns.push(quote! { #ident });

        let mut field_rules = extract_rules(&field.attrs)?;
        if field_rules.iter().any(|r| r == "skip") {
            continue;
        }
        if field_rules.is_empty() {
            field_rules = struct_rules.clone();
        }
        if field_rules.is_empty() {
            continue;
        }

        if !is_string_type(&ty) {
            let msg = format!(
                "Validation rules can only be applied to String fields: {}",
                ident
            );
            return Ok(quote! { compile_error!(#msg); });
        }

        for rule in field_rules {
            let ts = match rules::dispatch(&rule, &ident) {
                Some(ts) => ts,
                None => {
                    let msg = format!("Unknown rule `{}`", rule);
                    return Ok(quote! { compile_error!(#msg); });
                }
            };
            validations.push(ts);
        }
    }

    let error_doc = format!(
        "Returns [`{}`] if a field fails one of its `#[validate(...)]` rules.",
        quote!(#error_type).to_string().replace(' ', "")
    );
    let out = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Constructs a value from its fields, applying the `#[validate(...)]` rules.
            ///
            /// # Errors
            #[doc = #error_doc]
            #[allow(clippy::too_many_arguments)]
            pub fn new(
                #(#ctor_params),*
            ) -> Result<Self, #error_type> {

                #(
                    #validations
                )*

                Ok(Self {
                    #(#ctor_assigns),*
                })
            }
        }
    };

    Ok(out)
}
