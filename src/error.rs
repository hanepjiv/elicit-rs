// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/31
//  @date 2016/12/31

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
use std::fmt::Display;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// PoisonedRead
    PoisonedRead,
    /// PoisonedWrite
    PoisonedWrite,
    /// WouldBlock
    WouldBlock,
}
// ============================================================================
impl Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ Error::PoisonedRead
            | ref e @ Error::PoisonedWrite
            | ref e @ Error::WouldBlock => write!(f, "{:?}", e),
        }
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::PoisonedRead => "::elicit::Error::PoisonedRead",
            Error::PoisonedWrite => "::elicit::Error::PoisonedWrite",
            Error::WouldBlock => "::elicit::Error::WouldBlock",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::PoisonedRead
                | Error::PoisonedWrite
                | Error::WouldBlock => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<R> = ::std::result::Result<R, Box<StdError>>;
