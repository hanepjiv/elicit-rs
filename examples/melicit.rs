// -*- mode:rust; coding:utf-8-unix; -*-

//! melicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2025/04/06

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use elicit_macro as _;
// ----------------------------------------------------------------------------
#[cfg(feature = "parking_lot")]
use parking_lot as _;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) mod mine {
    use elicit::{Melicit, melicit_define};
    // ========================================================================
    #[melicit_define(mine_melicit)]
    pub(crate) trait Mine: Send {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) mine_melicit::author as melicit_author;
    pub(crate) use mine_melicit::user as melicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    pub(crate) struct X;
    // ------------------------------------------------------------------------
    impl Mine for X {
        fn action(&self) -> i32 {
            0_i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    // #[melicit_from_self_field(_fsf)] here
    pub(crate) struct Y {
        #[melicit_from_self_field] // or here
        _fsf: mine_melicit::author::MelicitFromSelfField,
        i: i32,
    }
    // ------------------------------------------------------------------------
    impl Y {
        pub(crate) fn new(a: i32) -> Self {
            Self {
                _fsf: mine_melicit::author::MelicitFromSelfField::default(),
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
    use super::mine::melicit_user::{
        Guard as MelicitGuard, LockError as MelicitLockError,
        TryLockError as MelicitTryLockError,
    };
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// enum Error
    #[expect(dead_code, reason = "checked")]
    #[derive(Debug)]
    pub(crate) enum Error<'a> {
        /// Elicit
        Elicit(elicit::Error),

        /// `MelicitLock`
        MelicitLock(MelicitLockError<MelicitGuard<'a>>),
        /// `MelicitTryLock`
        MelicitTryLock(MelicitTryLockError<MelicitGuard<'a>>),
    }
    // ========================================================================
    impl From<elicit::Error> for Error<'_> {
        fn from(e: elicit::Error) -> Self {
            Error::Elicit(e)
        }
    }
    // ------------------------------------------------------------------------
    impl<'s, 'a> From<MelicitLockError<MelicitGuard<'a>>> for Error<'s>
    where
        'a: 's,
    {
        fn from(e: MelicitLockError<MelicitGuard<'a>>) -> Self {
            Error::MelicitLock(e)
        }
    }
    // ------------------------------------------------------------------------
    impl<'s, 'a> From<MelicitTryLockError<MelicitGuard<'a>>> for Error<'s>
    where
        'a: 's,
    {
        fn from(e: MelicitTryLockError<MelicitGuard<'a>>) -> Self {
            Error::MelicitTryLock(e)
        }
    }
    // ========================================================================
    impl ::core::fmt::Display for Error<'_> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    // ========================================================================
    impl ::core::error::Error for Error<'static> {
        // ====================================================================
        fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
            match *self {
                Error::Elicit(ref e) => Some(e),
                Error::MelicitLock(_) | Error::MelicitTryLock(_) => None,
            }
        }
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// type Result
    pub(crate) type Result<'a, T> = ::core::result::Result<T, Error<'a>>;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
fn main() -> elicit::Result<()> {
    use mine::melicit_user::Melicit as MineMelicit;
    use mine::{X, Y};

    let mut e: MineMelicit;

    e = MineMelicit::new(X)?;

    if let Err(x) = e.with(|m| -> error::Result<'_, ()> {
        println!("{m:?}");
        assert!(m.action() == 0_i32, "default");
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    let y = Y::new(3);
    e = MineMelicit::new(y)?;

    if let Err(x) = e.try_with(|m| -> error::Result<'_, ()> {
        println!("{m:?}");
        assert!(m.action() == 3_i32, "user defined");
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    Ok(())
}
