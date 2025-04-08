// -*- mode:rust; coding:utf-8-unix; -*-

//! `elicit_derive.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/10
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::{
    DeriveInput, Error, Ident, Result, Span, ToTokens as _, TokenStream2,
    quote,
};
// ----------------------------------------------------------------------------
use crate::find_field_attr::find_field_attr;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// fn expand
pub(crate) fn expand(ast: DeriveInput) -> Result<TokenStream2> {
    let mut elicit_mod_author = Option::<TokenStream2>::default();
    let mut elicit_from_self_field = Option::<TokenStream2>::default();

    for attr in ast.attrs {
        match attr.meta.path().require_ident()?.to_string().as_str() {
            "elicit_mod_author" => {
                elicit_mod_author = Some(
                    attr.meta
                        .require_list()?
                        .parse_args::<syn::Path>()?
                        .into_token_stream(),
                );
            }
            "elicit_from_self_field" => {
                elicit_from_self_field = Some(
                    attr.meta
                        .require_list()?
                        .parse_args::<Ident>()?
                        .into_token_stream(),
                );
            }
            _ => {}
        }
    }

    find_field_attr(
        &ast.data,
        "elicit_from_self_field",
        &mut elicit_from_self_field,
    )?;

    if elicit_mod_author.is_none() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(Debug, Elicit)]
#[elicit_mod_author(ELICIT_MOD_AUTHOR)] // This attribute is necessary.
struct Derived{}
",
        ));
    }

    let ident = ast.ident;
    let elicit_impl = elicit_from_self_field.as_ref().map_or_else(
        || quote! { None },
        |x| quote! {self.#x.elicit_from_self()},
    );
    let _weak_assign_impl = elicit_from_self_field.as_ref().map_or_else(
        || quote! { Ok(()) },
        |x| quote! {self.#x._weak_assign(_weak)},
    );

    Ok(quote! {
    #[automatically_derived]
    impl #elicit_mod_author :: ElicitFromSelf for #ident {
    fn elicit_from_self(&self) ->
    Option<#elicit_mod_author :: Elicit> {
    #elicit_impl
    }
    }

    #[automatically_derived]
    impl #elicit_mod_author :: WeakAssign for #ident {
    fn _weak_assign(
    &mut self,
    _weak: #elicit_mod_author :: WeakElicitInner,
    ) -> elicit::Result<()> {
    #_weak_assign_impl
    }
    }
    })
}
// ============================================================================
#[cfg(test)]
#[expect(clippy::expect_used, clippy::unwrap_used, reason = "checked")]
mod tests {
    use super::*;
    use syn::parse2;
    // ========================================================================
    #[test]
    fn test_00() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                #[elicit_mod_author(ident_mod)]
                #[elicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap(),
        );
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_01() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                // #[elicit_mod_author(ident_mod)]
                #[elicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap_err(),
        );
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_02() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                #[elicit_mod_author(ident_mod)]
                // #[elicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap(),
        );
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_03() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                // #[elicit_mod_author(ident_mod)]
                // #[elicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap_err(),
        );
    }
}
