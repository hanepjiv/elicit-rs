// -*- mode:rust; coding:utf-8-unix; -*-

//! elicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/12/10

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use elicit_macro as _;
// ----------------------------------------------------------------------------
#[cfg(feature = "parking_lot")]
use parking_lot as _;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) mod mine {
    use elicit::{elicit_define, Elicit};
    // ========================================================================
    #[elicit_define(mine_elicit)]
    pub(crate) trait Mine {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) mine_elicit::author as elicit_author;
    pub(crate) use mine_elicit::user as elicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    pub(crate) struct X {}
    // ------------------------------------------------------------------------
    impl Mine for X {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    // #[elicit_from_self_field(_fsf)] // here
    pub(crate) struct Y {
        #[elicit_from_self_field] // or here
        _fsf: mine_elicit::author::ElicitFromSelfField,
        i: i32,
    }
    // ------------------------------------------------------------------------
    impl Y {
        pub(crate) fn new(a: i32) -> Self {
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

    e = MineElicit::new(X::default())?;

    e.try_with(|m| -> elicit::Result<()> {
        println!("{m:?}");

        assert!(m.action() == 0);

        Ok(())
    })?;

    let y = Y::new(1);
    e = MineElicit::new(y)?;

    e.try_with_mut(|m| -> elicit::Result<()> {
        println!("{m:?}");

        assert!(m.action() == 1);

        Ok(())
    })?;

    Ok(())
}
