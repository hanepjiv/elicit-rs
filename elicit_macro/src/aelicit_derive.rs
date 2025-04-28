// -*- mode:rust; coding:utf-8-unix; -*-

//! `aelicit_derive.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2025/04/28

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
    let mut aelicit_mod_author = Option::<TokenStream2>::default();
    let mut aelicit_from_self_field = Option::<TokenStream2>::default();

    for attr in ast.attrs {
        match attr.meta.path().require_ident()?.to_string().as_str() {
            "aelicit_mod_author" => {
                aelicit_mod_author = Some(
                    attr.meta
                        .require_list()?
                        .parse_args::<syn::Path>()?
                        .into_token_stream(),
                );
            }
            "aelicit_from_self_field" => {
                aelicit_from_self_field = Some(
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
        "aelicit_from_self_field",
        &mut aelicit_from_self_field,
    )?;

    if aelicit_mod_author.is_none() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(Debug, Aelicit)]
#[aelicit_mod_author(AELICIT_MOD_AUTHOR)] // This attribute is necessary.
struct Derived {}
",
        ));
    }

    let ident = ast.ident;
    let aelicit_impl = aelicit_from_self_field.as_ref().map_or_else(
        || quote! { None },
        |x| quote! {self.#x.aelicit_from_self()},
    );
    let _weak_assign_impl = aelicit_from_self_field.as_ref().map_or_else(
        || quote! { Ok(()) },
        |x| quote! {self.#x._weak_assign(_weak)},
    );

    Ok(quote! {
    #[automatically_derived]
    impl #aelicit_mod_author :: AelicitFromSelf for #ident {
    fn aelicit_from_self(&self) ->
    Option<#aelicit_mod_author :: Aelicit> {
    #aelicit_impl
    }
    }

    #[automatically_derived]
    impl #aelicit_mod_author :: WeakAssign for #ident {
    fn _weak_assign(
    &mut self,
    _weak: #aelicit_mod_author :: WeakAelicitInner,
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
    fn case_00() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                #[aelicit_mod_author(ident_mod)]
                #[aelicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap(),
        );
    }
    // ------------------------------------------------------------------------
    #[test]
    fn case_01() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                // #[aelicit_mod_author(ident_mod)]
                #[aelicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap_err(),
        );
    }
    // ------------------------------------------------------------------------
    #[test]
    fn case_02() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                #[aelicit_mod_author(ident_mod)]
                // #[aelicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap(),
        );
    }
    // ------------------------------------------------------------------------
    #[test]
    fn case_03() {
        drop(
            expand(
                parse2::<DeriveInput>(quote! {
                // #[aelicit_mod_author(ident_mod)]
                // #[aelicit_from_self_field(ident_field)]
                struct Orig {}
                })
                .expect("parse"),
            )
            .unwrap_err(),
        );
    }
}
