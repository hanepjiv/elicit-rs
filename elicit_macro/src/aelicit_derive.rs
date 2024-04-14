// -*- mode:rust; coding:utf-8-unix; -*-

//! aelicit_derive.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2024/04/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::*;
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
                )
            }
            _ => {}
        }
    }

    if aelicit_mod_author.is_none() {
        return Err(Error::new(
            Span::call_site(),
            format!(
                r###"Requires the following attributes.
#[derive(Aelicit)]
#[aelicit_mod_author(..)]              // {:?}
"###,
                aelicit_mod_author.map(|x| x.to_string()),
            ),
        ));
    }

    let ident = ast.ident;
    let aelicit_impl = match aelicit_from_self_field {
        Some(ref x) => quote! {self.#x.aelicit_from_self()},
        None => quote! { None },
    };
    let _weak_assign_impl = match aelicit_from_self_field {
        Some(ref x) => quote! {self.#x._weak_assign(_weak)},
        None => quote! {},
    };

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
                _weak: std::sync::Weak<std::sync::RwLock<Box<(
                    dyn #aelicit_mod_author :: AelicitBase)>>>,
            ) {
                #_weak_assign_impl
            }
        }
    })
}
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse2;
    // ========================================================================
    #[test]
    fn test_00() {
        assert!(expand(
            parse2::<DeriveInput>(quote! {
            #[aelicit_mod_author(ident_mod)]
            #[aelicit_from_self_field(ident_field)]
            struct Orig {}
            })
            .expect("parse")
        )
        .is_ok());
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_01() {
        assert!(expand(
            parse2::<DeriveInput>(quote! {
            // #[aelicit_mod_author(ident_mod)]
            #[aelicit_from_self_field(ident_field)]
            struct Orig {}
            })
            .expect("parse")
        )
        .is_err());
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_02() {
        assert!(expand(
            parse2::<DeriveInput>(quote! {
            #[aelicit_mod_author(ident_mod)]
            // #[aelicit_from_self_field(ident_field)]
            struct Orig {}
            })
            .expect("parse")
        )
        .is_ok());
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_03() {
        assert!(expand(
            parse2::<DeriveInput>(quote! {
            // #[aelicit_mod_author(ident_mod)]
            // #[aelicit_from_self_field(ident_field)]
            struct Orig {}
            })
            .expect("parse")
        )
        .is_err());
    }
}
