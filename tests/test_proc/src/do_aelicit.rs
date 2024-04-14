// -*- mode:rust; coding:utf-8-unix; -*-

//! do_aelicit.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2024/04/15

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub mod mine {
    use elicit::{aelicit_define, Aelicit};
    #[aelicit_define(mine_aelicit)]
    pub trait Mine: Sync + Send {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub use mine_aelicit::author as aelicit_author;
    pub use mine_aelicit::user as aelicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Aelicit)]
    #[aelicit_mod_author(mine_aelicit::author)]
    #[aelicit_from_self_field(_eefsf)]
    pub struct MineX {
        _eefsf: mine_aelicit::author::AelicitFromSelfField,
    }
    // ------------------------------------------------------------------------
    impl Mine for MineX {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Aelicit)]
    #[aelicit_mod_author(mine_aelicit::author)]
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
            use mine_aelicit::author::*;
            use std::sync::{Arc, RwLock};
            self._weak_assign(Arc::<RwLock<Box<dyn AelicitBase>>>::downgrade(
                &Arc::new(RwLock::new(Box::<MineX>::default())),
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
    use mine::aelicit_user::Error;
    use mine::aelicit_user::Aelicit as MineAelicit;
    use mine::{MineX, MineY};

    let mut e: MineAelicit;

    e = MineAelicit::new(MineX::default());
    e.with(|m| {
        println!("{:?}", m);

        assert!(m.action() == 0);

        Ok::<(), Error>(())
    })
    .expect("MineAelicit::with X");

    let y = MineY::new(1);
    // y.evil();

    e = MineAelicit::new(y);
    e.with(|m| {
        println!("{:?}", m);

        assert!(m.action() == 1);

        Ok::<(), Error>(())
    })
    .expect("MineAelicit::with Y");
}
