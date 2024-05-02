// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/05/03
//  @date 2024/05/03

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) mod do_aelicit;
pub(crate) mod do_elicit;
pub(crate) mod do_melicit;
// ============================================================================
#[cfg(feature = "parking_lot")]
mod error_parking_lot;
#[cfg(feature = "parking_lot")]
pub(crate) use error_parking_lot::Result;
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
mod error_default;
#[cfg(not(any(feature = "parking_lot",)))]
pub(crate) use error_default::Result;
