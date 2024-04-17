// -*- mode:rust; coding:utf-8-unix; -*-

//! find_field_attr.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/17
//  @date 2024/04/17

// ////////////////////////////////////////////////////////////////////////////
use crate::include::*;
// use  =======================================================================
pub(crate) fn find_field_attr<T>(
    data: &syn::Data,
    ident: T,
    ret: &mut Option<TokenStream2>,
) -> Result<()>
where
    T: AsRef<str>,
{
    if let syn::Data::Struct(x) = data {
        if let syn::Fields::Named(n) = &x.fields {
            for f in &n.named {
                for a in &f.attrs {
                    if ident.as_ref()
                        == a.meta.path().require_ident()?.to_string().as_str()
                    {
                        if let Some(ref i) = f.ident {
                            if ret.is_some() {
                                return Err(Error::new(
                                    Span::call_site(),
                                    format!(
                                        "from_self_field is already \
                                             exists. already: {}, new: {}",
                                        ret.as_ref().unwrap(),
                                        i
                                    ),
                                ));
                            } else {
                                *ret = Some(i.to_token_stream());
                            }
                        } else {
                            return Err(Error::new(
                                Span::call_site(),
                                "from_self_field: invalid ident.",
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
