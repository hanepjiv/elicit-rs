// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/31
//  @date 2024/12/03

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{error::Error as StdError, fmt::Display};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// `WouldBlock`,
    WouldBlock,
    /// `WeakAlreadyExists`
    WeakAlreadyExists,
    /// Borrow
    Borrow(std::cell::BorrowError),
    /// `BorrowMut`
    BorrowMut(std::cell::BorrowMutError),
}
// ============================================================================
impl Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl From<std::cell::BorrowError> for Error {
    fn from(e: std::cell::BorrowError) -> Self {
        Self::Borrow(e)
    }
}
// ----------------------------------------------------------------------------
impl From<std::cell::BorrowMutError> for Error {
    fn from(e: std::cell::BorrowMutError) -> Self {
        Self::BorrowMut(e)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Self::WouldBlock | Self::WeakAlreadyExists => None,
            Self::Borrow(ref e) => Some(e),
            Self::BorrowMut(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type `Result<T>`
pub type Result<T> = std::result::Result<T, Error>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::Error;
    // ========================================================================
    #[test]
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
