// -*- mode:rust; coding:utf-8-unix; -*-

//! aelicit.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2024/12/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use elicit_macro as _;
// ----------------------------------------------------------------------------
#[cfg(feature = "parking_lot")]
use parking_lot as _;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) mod mine {
    use elicit::{Aelicit, aelicit_define};
    #[aelicit_define(mine_aelicit)]
    pub trait Mine: Send + Sync {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) use mine_aelicit::author as aelicit_author;
    pub use mine_aelicit::user as aelicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Aelicit)]
    #[aelicit_mod_author(mine_aelicit::author)]
    pub struct X {}
    // ------------------------------------------------------------------------
    impl Mine for X {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Aelicit)]
    #[aelicit_mod_author(mine_aelicit::author)]
    //#[aelicit_from_self_field(_fsf)] // here
    pub struct Y {
        #[aelicit_from_self_field] // or here
        _fsf: mine_aelicit::author::AelicitFromSelfField,
        i: i32,
    }
    // ------------------------------------------------------------------------
    impl Y {
        pub(crate) fn new(a: i32) -> Self {
            Self {
                _fsf: mine_aelicit::author::AelicitFromSelfField::default(),
                i: a,
            }
        }
    }
    // ------------------------------------------------------------------------
    impl Mine for Y {
        fn action(&self) -> i32 {
            self.i
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(feature = "parking_lot")]
mod error {
    pub(crate) type Result<'a, T> = elicit::Result<T>;
}
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
mod error {
    use super::mine::aelicit_user::{
        LockError as AelicitLockError, ReadGuard as AelicitReadGuard,
        TryLockError as AelicitTryLockError, WriteGuard as AelicitWriteGuard,
    };
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// enum Error
    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Error<'a> {
        /// Elicit
        Elicit(elicit::Error),

        /// `AelicitLockRead`
        AelicitLockRead(AelicitLockError<AelicitReadGuard<'a>>),
        /// `AelicitLockWrite`
        AelicitLockWrite(AelicitLockError<AelicitWriteGuard<'a>>),
        /// `AelicitTryLockRead`
        AelicitTryLockRead(AelicitTryLockError<AelicitReadGuard<'a>>),
        /// `AelicitTryLockWrite`
        AelicitTryLockWrite(AelicitTryLockError<AelicitWriteGuard<'a>>),
    }
    // ========================================================================
    impl From<elicit::Error> for Error<'_> {
        fn from(e: elicit::Error) -> Self {
            Error::Elicit(e)
        }
    }
    // ------------------------------------------------------------------------
    impl<'s, 'a> From<AelicitLockError<AelicitReadGuard<'a>>> for Error<'s>
    where
        'a: 's,
    {
        fn from(e: AelicitLockError<AelicitReadGuard<'a>>) -> Self {
            Error::AelicitLockRead(e)
        }
    }
    // ------------------------------------------------------------------------
    impl<'s, 'a> From<AelicitLockError<AelicitWriteGuard<'a>>> for Error<'s>
    where
        'a: 's,
    {
        fn from(e: AelicitLockError<AelicitWriteGuard<'a>>) -> Self {
            Error::AelicitLockWrite(e)
        }
    }
    // ------------------------------------------------------------------------
    impl<'s, 'a> From<AelicitTryLockError<AelicitReadGuard<'a>>> for Error<'s>
    where
        'a: 's,
    {
        fn from(e: AelicitTryLockError<AelicitReadGuard<'a>>) -> Self {
            Error::AelicitTryLockRead(e)
        }
    }
    // ------------------------------------------------------------------------
    impl<'s, 'a> From<AelicitTryLockError<AelicitWriteGuard<'a>>> for Error<'s>
    where
        'a: 's,
    {
        fn from(e: AelicitTryLockError<AelicitWriteGuard<'a>>) -> Self {
            Error::AelicitTryLockWrite(e)
        }
    }
    // ========================================================================
    impl ::std::fmt::Display for Error<'_> {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    // ========================================================================
    impl ::std::error::Error for Error<'static> {
        // ====================================================================
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match *self {
                Error::Elicit(ref e) => Some(e),
                Error::AelicitLockRead(_)
                | Error::AelicitLockWrite(_)
                | Error::AelicitTryLockRead(_)
                | Error::AelicitTryLockWrite(_) => None,
            }
        }
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// type Result
    pub type Result<'a, T> = ::std::result::Result<T, Error<'a>>;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn main() -> elicit::Result<()> {
    use mine::aelicit_user::Aelicit as MineAelicit;
    use mine::{X, Y};

    let mut e: MineAelicit;

    e = MineAelicit::new(X::default())?;

    if let Err(x) = e.with(|m| -> error::Result<'_, ()> {
        println!("{m:?}");
        assert!(m.action() == 0);
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    let y = Y::new(2);
    e = MineAelicit::new(y)?;

    if let Err(e) = e.try_with(|m| -> error::Result<'_, ()> {
        println!("{m:?}");
        assert!(m.action() == 2);
        Ok(())
    }) {
        eprintln!("{e:?}");
    }

    Ok(())
}
