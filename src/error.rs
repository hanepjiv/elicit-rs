// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/31
//  @date 2024/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
use std::fmt::Display;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
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
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::PoisonedRead | Error::PoisonedWrite | Error::WouldBlock => {
                None
            }
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::Error;
    // ========================================================================
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
