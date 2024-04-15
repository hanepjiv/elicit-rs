// -*- mode:rust; coding:utf-8-unix; -*-

//! do_elicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/04/15

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub mod mine {
    use elicit::{elicit_define, Elicit};
    // ========================================================================
    #[elicit_define(mine_elicit)]
    pub trait Mine {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub use mine_elicit::author as elicit_author;
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
    #[elicit_from_self_field(_fsf)]
    pub struct MineY {
        _fsf: mine_elicit::author::ElicitFromSelfField,
        pub i: i32,
    }
    // ------------------------------------------------------------------------
    impl MineY {
        pub fn new(a: i32) -> Self {
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
        #[allow(dead_code)]
        pub fn evil(&mut self) -> elicit::Result<()> {
            use mine_elicit::author::*;
            use std::{cell::RefCell, rc::Rc};
            self._weak_assign(Rc::<RefCell<Box<dyn ElicitBase>>>::downgrade(
                &Rc::new(RefCell::new(Box::<MineX>::default())),
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
pub fn fire() {
    use elicit::Error;
    use mine::elicit_user::Elicit as MineElicit;
    use mine::{MineX, MineY};

    let mut e: MineElicit;

    e = MineElicit::new(MineX::default());
    e.try_with(|m| {
        println!("{:?}", m);

        assert!(m.action() == 0);

        Ok::<(), Error>(())
    })
    .expect("MineElicit::with X");

    let y = MineY::new(1);

    // eprintln!("{:?}", y.evil());

    e = MineElicit::new(y);

    e.try_with_mut(|m| {
        println!("{:?}", m);

        assert!(m.action() == 1);

        Ok::<(), Error>(())
    })
    .expect("MineElicit::with Y");
}
