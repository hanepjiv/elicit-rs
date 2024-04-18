// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/18
//  @date 2024/04/19

// ////////////////////////////////////////////////////////////////////////////
// attributes  ================================================================
#![allow(box_pointers)]
// use  =======================================================================
use super::do_aelicit::mine::aelicit_user::{
    LockError as AelicitLockError, ReadGuard as AelicitReadGuard,
    TryLockError as AelicitTryLockError, WriteGuard as AelicitWriteGuard,
};
use super::do_melicit::mine::melicit_user::{
    Guard as MelicitGuard, LockError as MelicitLockError,
    TryLockError as MelicitTryLockError,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Error<'a> {
    /// Elicit
    Elicit(elicit::Error),

    /// AelicitLockRead
    AelicitLockRead(AelicitLockError<AelicitReadGuard<'a>>),
    /// AelicitLockWrite
    AelicitLockWrite(AelicitLockError<AelicitWriteGuard<'a>>),
    /// AelicitTryLockRead
    AelicitTryLockRead(AelicitTryLockError<AelicitReadGuard<'a>>),
    /// AelicitTryLockWrite
    AelicitTryLockWrite(AelicitTryLockError<AelicitWriteGuard<'a>>),

    /// MelicitLock
    MelicitLock(MelicitLockError<MelicitGuard<'a>>),
    /// MelicitTryLockRead
    MelicitTryLock(MelicitTryLockError<MelicitGuard<'a>>),
}
// ============================================================================
impl From<elicit::Error> for Error<'_> {
    fn from(e: elicit::Error) -> Self {
        Error::Elicit(e)
    }
}
// ----------------------------------------------------------------------------
impl<'s, 'a> From<AelicitLockError<AelicitReadGuard<'a>>> for Error<'s>
where
    'a: 's,
{
    fn from(e: AelicitLockError<AelicitReadGuard<'a>>) -> Self {
        Error::AelicitLockRead(e)
    }
}
// ----------------------------------------------------------------------------
impl<'s, 'a> From<AelicitLockError<AelicitWriteGuard<'a>>> for Error<'s>
where
    'a: 's,
{
    fn from(e: AelicitLockError<AelicitWriteGuard<'a>>) -> Self {
        Error::AelicitLockWrite(e)
    }
}
// ----------------------------------------------------------------------------
impl<'s, 'a> From<AelicitTryLockError<AelicitReadGuard<'a>>> for Error<'s>
where
    'a: 's,
{
    fn from(e: AelicitTryLockError<AelicitReadGuard<'a>>) -> Self {
        Error::AelicitTryLockRead(e)
    }
}
// ----------------------------------------------------------------------------
impl<'s, 'a> From<AelicitTryLockError<AelicitWriteGuard<'a>>> for Error<'s>
where
    'a: 's,
{
    fn from(e: AelicitTryLockError<AelicitWriteGuard<'a>>) -> Self {
        Error::AelicitTryLockWrite(e)
    }
}
// ----------------------------------------------------------------------------
impl<'s, 'a> From<MelicitLockError<MelicitGuard<'a>>> for Error<'s>
where
    'a: 's,
{
    fn from(e: MelicitLockError<MelicitGuard<'a>>) -> Self {
        Error::MelicitLock(e)
    }
}
// ----------------------------------------------------------------------------
impl<'s, 'a> From<MelicitTryLockError<MelicitGuard<'a>>> for Error<'s>
where
    'a: 's,
{
    fn from(e: MelicitTryLockError<MelicitGuard<'a>>) -> Self {
        Error::MelicitTryLock(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error<'static> {
    // ========================================================================
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Elicit(ref e) => Some(e),
            Error::AelicitLockRead(_) => None,
            Error::AelicitLockWrite(_) => None,
            Error::AelicitTryLockRead(_) => None,
            Error::AelicitTryLockWrite(_) => None,
            Error::MelicitLock(_) => None,
            Error::MelicitTryLock(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub(crate) type Result<'a, T> = ::std::result::Result<T, Error<'a>>;
