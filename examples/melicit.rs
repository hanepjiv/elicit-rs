// -*- mode:rust; coding:utf-8-unix; -*-

//! melicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/09/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use elicit_macro as _;
// ----------------------------------------------------------------------------
#[cfg(feature = "parking_lot")]
use parking_lot as _;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) mod mine {
    use elicit::{melicit_define, Melicit};
    // ========================================================================
    #[melicit_define(mine_melicit)]
    pub trait Mine: Send {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) mine_melicit::author as melicit_author;
    pub use mine_melicit::user as melicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    pub struct MineX {}
    // ------------------------------------------------------------------------
    impl Mine for MineX {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    // #[melicit_from_self_field(_fsf)] here
    pub struct MineY {
        #[melicit_from_self_field] // or here
        _fsf: mine_melicit::author::MelicitFromSelfField,
        i: i32,
    }
    // ------------------------------------------------------------------------
    impl MineY {
        pub(crate) fn new(a: i32) -> Self {
            Self {
                _fsf: Default::default(),
                i: a,
            }
        }
    }
    // ------------------------------------------------------------------------
    impl Mine for MineY {
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
    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Error<'a> {
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
                Error::MelicitLock(_) => None,
                Error::MelicitTryLock(_) => None,
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
    use mine::melicit_user::Melicit as MineMelicit;
    use mine::{MineX, MineY};

    let mut e: MineMelicit;

    e = MineMelicit::new(MineX::default())?;

    if let Err(x) = e.with(|m| -> error::Result<'_, ()> {
        println!("{m:?}");
        assert!(m.action() == 0);
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    let y = MineY::new(3);
    e = MineMelicit::new(y)?;

    if let Err(x) = e.try_with(|m| -> error::Result<'_, ()> {
        println!("{m:?}");
        assert!(m.action() == 3);
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    Ok(())
}
