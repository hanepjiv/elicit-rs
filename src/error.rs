// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/31
//  @date 2025/04/28

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use core::{error::Error as StdError, fmt::Display};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// `WouldBlock`,
    WouldBlock,
    /// `WeakAlreadyExists`
    WeakAlreadyExists,
    /// Borrow
    Borrow(core::cell::BorrowError),
    /// `BorrowMut`
    BorrowMut(core::cell::BorrowMutError),
}
// ============================================================================
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl From<core::cell::BorrowError> for Error {
    #[inline]
    fn from(e: core::cell::BorrowError) -> Self {
        Self::Borrow(e)
    }
}
// ----------------------------------------------------------------------------
impl From<core::cell::BorrowMutError> for Error {
    #[inline]
    fn from(e: core::cell::BorrowMutError) -> Self {
        Self::BorrowMut(e)
    }
}
// ============================================================================
impl StdError for Error {
    #[inline]
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
pub type Result<T> = core::result::Result<T, Error>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::Error;
    // ========================================================================
    #[test]
    const fn send() {
        const fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
    }
}
