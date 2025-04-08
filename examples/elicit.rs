// -*- mode:rust; coding:utf-8-unix; -*-

//! elicit.rs

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
    use elicit::{Elicit, elicit_define};
    // ========================================================================
    #[elicit_define(mine_elicit)]
    pub(super) trait Mine {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(super) mine_elicit::author as elicit_author;
    pub(super) use mine_elicit::user as elicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    pub(super) struct X;
    // ------------------------------------------------------------------------
    impl Mine for X {
        fn action(&self) -> i32 {
            0_i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    // #[elicit_from_self_field(_fsf)] // here
    pub(super) struct Y {
        #[elicit_from_self_field] // or here
        _fsf: mine_elicit::author::ElicitFromSelfField,
        i: i32,
    }
    // ------------------------------------------------------------------------
    impl Y {
        pub(super) fn new(a: i32) -> Self {
            Self {
                _fsf: mine_elicit::author::ElicitFromSelfField::default(),
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
fn main() -> elicit::Result<()> {
    use mine::elicit_user::Elicit as MineElicit;
    use mine::{X, Y};

    let mut e: MineElicit;

    e = MineElicit::new(X)?;

    e.try_with(|m| -> elicit::Result<()> {
        println!("{m:?}");

        assert!(m.action() == 0_i32, "default");

        Ok(())
    })?;

    let y = Y::new(1);
    e = MineElicit::new(y)?;

    e.try_with_mut(|m| -> elicit::Result<()> {
        println!("{m:?}");

        assert!(m.action() == 1_i32, "user defined");

        Ok(())
    })?;

    Ok(())
}
