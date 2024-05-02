// -*- mode:rust; coding:utf-8-unix; -*-

//! error_parking_lot.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/18
//  @date 2024/05/03

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Error {
    /// Elicit
    Elicit(elicit::Error),
}
// ============================================================================
impl From<elicit::Error> for Error {
    fn from(e: elicit::Error) -> Self {
        Error::Elicit(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Elicit(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// type Result
///
/// pub(crate) type Result<T> = ::std::result::Result<T, Error>;
///
/// The above is more appropriate, but only to make the code common.
///
pub(crate) type Result<'a, T> = ::std::result::Result<T, Error>;
