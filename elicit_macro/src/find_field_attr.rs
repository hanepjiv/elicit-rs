// -*- mode:rust; coding:utf-8-unix; -*-

//! `find_field_attr.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/17
//  @date 2024/11/30

// ////////////////////////////////////////////////////////////////////////////
use crate::include::{Error, Ident, Result, Span, ToTokens, TokenStream2};
// use  =======================================================================
pub(crate) fn find_field_attr<T: ?Sized>(
    data: &syn::Data,
    ident: &T,
    ret: &mut Option<TokenStream2>,
) -> Result<()>
where
    Ident: PartialEq<T>,
{
    let syn::Data::Struct(x) = data else {
        return Ok(());
    };

    let syn::Fields::Named(n) = &x.fields else {
        return Ok(());
    };

    for f in &n.named {
        for a in &f.attrs {
            if !a.meta.path().is_ident(ident) {
                continue;
            }

            let Some(ref i) = f.ident else {
                return Err(Error::new(
                    Span::call_site(),
                    "from_self_field: invalid ident.",
                ));
            };

            if ret.is_some() {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "from_self_field is already exists. \
                         already: {}, new: {}",
                        ret.as_ref().unwrap(),
                        i
                    ),
                ));
            }

            *ret = Some(i.to_token_stream());
        }
    }

    Ok(())
}
