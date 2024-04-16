// -*- mode:rust; coding:utf-8-unix; -*-

//! do_melicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/04/16

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
    #[melicit_from_self_field(_fsf)]
    pub(crate) struct MineY {
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
            use mine_melicit::author::*;
            use std::sync::{Arc, Mutex};
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
pub(crate) fn fire() -> elicit::Result<()> {
    use mine::melicit_user::Melicit as MineMelicit;
    use mine::{MineX, MineY};

    let mut e: MineMelicit;

    e = MineMelicit::new(MineX::default())?;

    e.with(|x| {
        println!("{:?}", x);
        assert!(x.action() == 0);
        Ok::<(), elicit::Error>(())
    })?;

    let y = MineY::new(3);
    // eprintln!("{:?}", y.evil());

    e = MineMelicit::new(y)?;

    e.with(|x| {
        println!("{:?}", x);
        assert!(x.action() == 3);
        Ok::<(), elicit::Error>(())
    })?;

    Ok(())
}
