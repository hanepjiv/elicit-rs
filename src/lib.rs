// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/08
//  @date 2025/01/20

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
// ////////////////////////////////////////////////////////////////////////////
// mod  =======================================================================
mod error;
// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
#[cfg(feature = "parking_lot")]
pub use parking_lot::{
    Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
pub use std::sync::{
    Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
// ============================================================================
pub use self::error::{Error, Result};
pub use elicit_macro::{
    Aelicit, Elicit, Melicit, aelicit_define, elicit_define, melicit_define,
};
