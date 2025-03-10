// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/10
//  @date 2025/03/02

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
// ////////////////////////////////////////////////////////////////////////////
// mod  =======================================================================
pub(crate) mod find_field_attr;
// ============================================================================
///
/// mod include
///
/// `proc_macro::TokenStream` is not included in `crate::include` to suppress
/// "procedural macro API is used outside of a procedural macro".
///
pub(crate) mod include {
    // common  ----------------------------------------------------------------
    pub(crate) use proc_macro2::{Span, TokenStream as TokenStream2};
    pub(crate) use quote::{ToTokens, quote};
    pub(crate) use syn::{Error, parse_macro_input};

    pub(crate) type Result<T> = std::result::Result<T, Error>;

    #[inline]
    pub(crate) fn into_tokens(
        res: Result<TokenStream2>,
    ) -> proc_macro::TokenStream {
        res.unwrap_or_else(Error::into_compile_error).into()
    }

    // for elicit_macro  ------------------------------------------------------
    pub(crate) use syn::{DeriveInput, Ident, ItemTrait};
}
// ============================================================================
mod aelicit_derive;
mod elicit_define;
mod elicit_derive;
mod melicit_derive;
// ============================================================================
#[cfg(feature = "parking_lot")]
mod feature_parking_lot;
#[cfg(feature = "parking_lot")]
use feature_parking_lot::{aelicit_define, melicit_define};
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
mod feature_default;
#[cfg(not(any(feature = "parking_lot",)))]
use feature_default::{aelicit_define, melicit_define};
// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::{
    DeriveInput, Ident, ItemTrait, into_tokens, parse_macro_input,
};
use proc_macro::TokenStream;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// ```compile_fail
/// #[elicit_define(MODULE)]
/// trait Base {..}
/// ```
///
#[proc_macro_attribute]
pub fn elicit_define(attr: TokenStream, item: TokenStream) -> TokenStream {
    into_tokens(elicit_define::expand(
        &parse_macro_input!(attr as Ident),
        parse_macro_input!(item as ItemTrait),
    ))
}
// ============================================================================
///
/// ```compile_fail
/// #[derive(Debug, Elicit)]
/// #[elicit_mod_author(MODULE)]        // required
/// #[elicit_from_self_field(_fsf)]     // option here
/// struct Derived {
///   #[elicit_from_self_field]         // or here
///   _fsf: ElicitFromSelfField,
/// }
/// ```
///
#[proc_macro_derive(
    Elicit,
    attributes(elicit_mod_author, elicit_from_self_field)
)]
pub fn on_elicit_derive(ts: TokenStream) -> TokenStream {
    into_tokens(elicit_derive::expand(parse_macro_input!(ts as DeriveInput)))
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// ```compile_fail
/// #[aelicit_define(MODULE)]
/// trait Base {..}
/// ```
///
#[proc_macro_attribute]
pub fn aelicit_define(attr: TokenStream, item: TokenStream) -> TokenStream {
    into_tokens(aelicit_define::expand(
        &parse_macro_input!(attr as Ident),
        parse_macro_input!(item as ItemTrait),
    ))
}
// ============================================================================
///
/// ```compile_fail
/// #[derive(Debug, Aelicit)]
/// #[aelicit_mod_author(MODULE)]        // required
/// #[aelicit_from_self_field(_fsf)]     // option here
/// struct Derived {
///   #[aelicit_from_self_field]         // or here
///   _fsf: AelicitFromSelfField,
/// }
/// ```
///
#[proc_macro_derive(
    Aelicit,
    attributes(aelicit_mod_author, aelicit_from_self_field)
)]
pub fn on_aelicit_derive(ts: TokenStream) -> TokenStream {
    into_tokens(aelicit_derive::expand(parse_macro_input!(
        ts as DeriveInput
    )))
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// ```compile_fail
/// #[melicit_define(MODULE)]
/// trait Base {..}
/// ```
///
#[proc_macro_attribute]
pub fn melicit_define(attr: TokenStream, item: TokenStream) -> TokenStream {
    into_tokens(melicit_define::expand(
        &parse_macro_input!(attr as Ident),
        parse_macro_input!(item as ItemTrait),
    ))
}
// ============================================================================
///
/// ```compile_fail
/// #[derive(Debug, Melicit)]
/// #[melicit_mod_author(MODULE)]        // required
/// #[melicit_from_self_field(_fsf)]     // option here
/// struct Derived {
///   #[melicit_from_self_field]         // or here
///   _fsf: MelicitFromSelfField,
/// }
/// ```
///
#[proc_macro_derive(
    Melicit,
    attributes(melicit_mod_author, melicit_from_self_field)
)]
pub fn on_melicit_derive(ts: TokenStream) -> TokenStream {
    into_tokens(melicit_derive::expand(parse_macro_input!(
        ts as DeriveInput
    )))
}
