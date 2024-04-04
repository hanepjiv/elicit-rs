// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/08
//  @date 2024/04/04

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![forbid(
    clippy::all,
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2021_compatibility
)]
#![warn(unused, warnings)]
// mod  =======================================================================
pub mod aelicit;
pub mod elicit;
mod error;
// use  =======================================================================
pub use self::error::Error;
