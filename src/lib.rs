// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/08
//  @date 2016/12/31

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(fat_ptr_transmutes, missing_copy_implementations,
        missing_debug_implementations, missing_docs, unstable_features,
        unused_results, unused_import_braces, variant_size_differences)]
#![warn(unused_qualifications, unused_extern_crates, warnings)]
#![allow(box_pointers, trivial_casts, trivial_numeric_casts, unsafe_code)]
// use  =======================================================================
pub use self::error::{ Result, Error, };
// mod  =======================================================================
#[macro_use] pub mod error;
#[macro_use] pub mod elicit;
#[macro_use] pub mod aelicit;
