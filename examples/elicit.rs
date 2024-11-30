// -*- mode:rust; coding:utf-8-unix; -*-

//! elicit.rs

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
    use elicit::{elicit_define, Elicit};
    // ========================================================================
    #[elicit_define(mine_elicit)]
    pub trait Mine {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) mine_elicit::author as elicit_author;
    pub use mine_elicit::user as elicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    pub struct MineX {}
    // ------------------------------------------------------------------------
    impl Mine for MineX {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Elicit)]
    #[elicit_mod_author(mine_elicit::author)]
    // #[elicit_from_self_field(_fsf)] // here
    pub struct MineY {
        #[elicit_from_self_field] // or here
        _fsf: mine_elicit::author::ElicitFromSelfField,
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
fn main() -> elicit::Result<()> {
    use mine::elicit_user::Elicit as MineElicit;
    use mine::{MineX, MineY};

    let mut e: MineElicit;

    e = MineElicit::new(MineX::default())?;

    e.try_with(|m| -> elicit::Result<()> {
        println!("{m:?}");

        assert!(m.action() == 0);

        Ok(())
    })?;

    let y = MineY::new(1);
    e = MineElicit::new(y)?;

    e.try_with_mut(|m| -> elicit::Result<()> {
        println!("{m:?}");

        assert!(m.action() == 1);

        Ok(())
    })?;

    Ok(())
}
