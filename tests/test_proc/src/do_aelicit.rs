// -*- mode:rust; coding:utf-8-unix; -*-

//! do_aelicit.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2024/04/16

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) mod mine {
    use elicit::{aelicit_define, Aelicit, Result};
    #[aelicit_define(mine_aelicit)]
    pub(crate) trait Mine: Send + Sync {
        fn action(&self) -> i32;
    }
    // ------------------------------------------------------------------------
    // pub(crate) use mine_aelicit::author as aelicit_author;
    pub(crate) use mine_aelicit::user as aelicit_user;
    // ========================================================================
    #[derive(Debug, Default, Clone, Aelicit)]
    #[aelicit_mod_author(mine_aelicit::author)]
    pub(crate) struct MineX {}
    // ------------------------------------------------------------------------
    impl Mine for MineX {
        fn action(&self) -> i32 {
            0i32
        }
    }
    // ========================================================================
    #[derive(Debug, Clone, Aelicit)]
    #[aelicit_mod_author(mine_aelicit::author)]
    #[aelicit_from_self_field(_fsf)]
    pub(crate) struct MineY {
        _fsf: mine_aelicit::author::AelicitFromSelfField,
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
        pub(crate) fn evil(&mut self) -> Result<()> {
            use mine_aelicit::author::*;
            use std::sync::{Arc, RwLock};
            self._weak_assign(Arc::<RwLock<Box<dyn AelicitBase>>>::downgrade(
                &Arc::new(RwLock::new(Box::<MineX>::default())),
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
pub(crate) fn fire() {
    use elicit::Error;
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

    let y = MineY::new(2);
    // eprintln!("{:?}", y.evil());

    e = MineAelicit::new(y);
    e.with(|m| {
        println!("{:?}", m);

        assert!(m.action() == 2);

        Ok::<(), Error>(())
    })
    .expect("MineAelicit::with Y");
}
