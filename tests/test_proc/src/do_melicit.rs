// -*- mode:rust; coding:utf-8-unix; -*-

//! do_melicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/04/15

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub mod mine {
    use elicit::{melicit_define, Melicit};
    // ========================================================================
    #[melicit_define(mine_melicit)]
    pub trait Mine {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub use mine_melicit::author as melicit_author;
    pub use mine_melicit::user as melicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    #[melicit_from_self_field(_eefsf)]
    pub struct MineX {
        _eefsf: mine_melicit::author::MelicitFromSelfField,
    }
    // ------------------------------------------------------------------------
    impl Mine for MineX {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Melicit)]
    #[melicit_mod_author(mine_melicit::author)]
    pub struct MineY {
        pub i: i32,
    }
    // ------------------------------------------------------------------------
    impl MineY {
        pub fn new(a: i32) -> Self {
            MineY { i: a }
        }

        ///
        /// fn evil
        ///
        /// It is not possible to suppress calls to _weak_assign within
        /// the same module.
        ///
        #[allow(dead_code)]
        pub fn evil(&mut self) {
            use mine_melicit::author::*;
            use std::sync::{Arc, Mutex};
            self._weak_assign(Arc::<Mutex<Box<dyn MelicitBase>>>::downgrade(
                &Arc::new(Mutex::new(Box::<MineX>::default())),
            ));
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
    use mine::melicit_user::Melicit as MineMelicit;
    use mine::{MineX, MineY};

    let mut e: MineMelicit;

    e = MineMelicit::new(MineX::default());
    {
        match e.lock() {
            Err(x) => {
                panic!("MineX: {x:?}");
            }
            Ok(x) => {
                println!("{:?}", x);
                assert!(x.action() == 0);
            }
        };
    }

    let y = MineY::new(3);
    // y.evil();

    e = MineMelicit::new(y);
    {
        match e.lock() {
            Err(x) => {
                panic!("MineY: {x:?}");
            }
            Ok(x) => {
                println!("{:?}", x);
                assert!(x.action() == 3);
            }
        };
    }
}
