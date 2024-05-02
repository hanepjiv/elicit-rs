// -*- mode:rust; coding:utf-8-unix; -*-

//! melicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/05/03

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
    pub(crate) trait Mine: Send {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) mine_melicit::author as melicit_author;
    pub(crate) use mine_melicit::user as melicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    pub(crate) struct MineX {}
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
    pub(crate) struct MineY {
        #[melicit_from_self_field] // or here
        _fsf: mine_melicit::author::MelicitFromSelfField,
        i: i32,
    }
    // ------------------------------------------------------------------------
    impl MineY {
        pub(crate) fn new(a: i32) -> Self {
            MineY {
                _fsf: Default::default(),
                i: a,
            }
        }

        ///
        /// fn evil
        ///
        /// It is not possible to suppress calls to _weak_assign within
        /// the same module.
        ///
        #[allow(box_pointers, dead_code)]
        pub(crate) fn evil(&mut self) -> elicit::Result<()> {
            use elicit::Mutex;
            use mine_melicit::author::*;
            use std::sync::Arc;
            self._weak_assign(Arc::<Mutex<Box<dyn MelicitBase>>>::downgrade(
                &Arc::new(Mutex::new(Box::<MineX>::default())),
            ))
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
    pub(crate) enum Error<'a> {
        /// Elicit
        Elicit(elicit::Error),

        /// MelicitLock
        MelicitLock(MelicitLockError<MelicitGuard<'a>>),
        /// MelicitTryLock
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
            write!(f, "{:?}", self)
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
    pub(crate) type Result<'a, T> = ::std::result::Result<T, Error<'a>>;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[allow(box_pointers)]
fn main() -> elicit::Result<()> {
    use mine::melicit_user::Melicit as MineMelicit;
    use mine::{MineX, MineY};

    let mut e: MineMelicit;

    e = MineMelicit::new(MineX::default())?;

    if let Err(x) = e.with(|m| -> error::Result<'_, ()> {
        println!("{:?}", m);
        assert!(m.action() == 0);
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    let y = MineY::new(3);
    // eprintln!("{:?}", y.evil());

    e = MineMelicit::new(y)?;

    if let Err(x) = e.try_with(|m| -> error::Result<'_, ()> {
        println!("{:?}", m);
        assert!(m.action() == 3);
        Ok(())
    }) {
        eprintln!("{x:?}");
    }

    Ok(())
}
