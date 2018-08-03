// -*- mode:rust; coding:utf-8-unix; -*-

//! elicit.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/08/18
//  @date 2018/06/01

//! # Examples
//!
//! ```
//! #[macro_use] extern crate elicit;
//!
//! elicit_define!(elicit_my_trait, MyTrait);
//! use self::elicit_my_trait::Elicit
//!     as MyTraitElicit;
//! use self::elicit_my_trait::WeakElicit
//!     as MyTraitWeakElicit;
//! use self::elicit_my_trait::EnableElicitFromSelf
//!     as MyTraitEnableElicitFromSelf;
//! use self::elicit_my_trait::EnableElicitFromSelfField
//!     as MyTraitEnableElicitFromSelfField;
//!
//! pub trait MyTrait: ::std::fmt::Debug + MyTraitEnableElicitFromSelf {
//!     fn my_function(&self) -> i32;
//! }
//!
//! #[derive( Debug, )]
//! struct MyStruct {
//!     _eefsf:        MyTraitEnableElicitFromSelfField,
//!     my_field:     i32,
//! }
//! impl MyTraitEnableElicitFromSelf for MyStruct {
//!     enable_elicit_from_self_delegate!(MyTrait, MyTraitElicit, _eefsf);
//! }
//! impl MyTrait for MyStruct {
//!     fn my_function(&self) -> i32 { self.my_field }
//! }
//!
//! #[derive( Debug, )]
//! struct MyStructUnuseEnableElicitFromSelf {
//!     my_field:     i32,
//! }
//! impl MyTraitEnableElicitFromSelf for MyStructUnuseEnableElicitFromSelf {
//!     enable_elicit_from_self_delegate!(MyTrait, MyTraitElicit);
//! }
//! impl MyTrait for MyStructUnuseEnableElicitFromSelf {
//!     fn my_function(&self) -> i32 { self.my_field }
//! }
//!
//! fn main() {
//!     let my0 = MyTraitElicit::new(MyStruct{
//!         _eefsf: MyTraitEnableElicitFromSelfField::default(),
//!         my_field: 0i32,
//!     });
//!     let my1 = MyTraitElicit::new(MyStructUnuseEnableElicitFromSelf{
//!         my_field: 1i32,
//!     });
//! }
//! ```

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// elicit_define!
#[macro_export]
macro_rules! elicit_define {
    ($modname:ident, $base:ident) => {
        // ////////////////////////////////////////////////////////////////////
        // ====================================================================
        #[allow(unreachable_pub)]
        pub mod $modname {
            //! $modname
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            use super::$base;
            // ================================================================
            use std::any::Any;
            use std::cell::RefCell;
            use std::convert::From;
            use std::fmt::Debug;
            use std::rc::{Rc, Weak};
            use std::result::Result as StdResult;
            use $crate::Error;
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct Elicit
            #[derive(Debug, Clone)]
            pub struct Elicit(Rc<RefCell<Box<dyn $base>>>);
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct WeakElicit
            #[derive(Debug, Clone)]
            pub struct WeakElicit(Weak<RefCell<Box<dyn $base>>>);
            // ================================================================
            impl WeakElicit {
                // ============================================================
                /// fn upgrade
                pub fn upgrade(&self) -> Option<Elicit> {
                    self.0.upgrade().map(Elicit)
                }
            }
            // ================================================================
            impl From<Elicit> for WeakElicit {
                fn from(x: Elicit) -> WeakElicit {
                    x.weak()
                }
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// trait EnableElicitFromSelf
            pub trait EnableElicitFromSelf: Debug {
                // ============================================================
                /// elicit
                fn elicit(&self) -> Option<Elicit>;
                // ------------------------------------------------------------
                /// _weak_assign
                fn _weak_assign(
                    &mut self,
                    weak: Weak<RefCell<Box<dyn $base>>>,
                );
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct EnableElicitFromSelfField
            #[derive(Debug, Clone, Default)]
            pub struct EnableElicitFromSelfField {
                /// Weak
                _weak: Option<Weak<RefCell<Box<dyn $base>>>>,
            }
            // ================================================================
            impl EnableElicitFromSelf for EnableElicitFromSelfField {
                // ============================================================
                /// elicit_from_self
                fn elicit(&self) -> Option<Elicit> {
                    match self._weak {
                        Some(ref x) => x.upgrade().map(Elicit),
                        None => None,
                    }
                }
                // ------------------------------------------------------------
                /// _weak_assign
                fn _weak_assign(
                    &mut self,
                    weak: Weak<RefCell<Box<dyn $base>>>,
                ) {
                    self._weak = Some(weak)
                }
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            impl Elicit {
                // ============================================================
                /// new
                pub fn new<T>(val: T) -> Self
                where
                    T: Any + $base,
                    dyn $base: Debug + EnableElicitFromSelf,
                {
                    let rc =
                        Rc::new(RefCell::new(Box::new(val) as Box<dyn $base>));
                    rc.as_ref().borrow_mut()._weak_assign(Rc::downgrade(&rc));
                    Elicit(rc)
                }
                // ============================================================
                /// weak
                pub fn weak(&self) -> WeakElicit {
                    WeakElicit(Rc::downgrade(&self.0))
                }
                // ============================================================
                /// with
                pub fn with<T, E>(
                    &self,
                    f: impl FnOnce(&dyn $base) -> StdResult<T, E>,
                ) -> StdResult<T, E>
                where
                    E: From<Error>,
                    dyn $base: Debug + EnableElicitFromSelf,
                {
                    f(&(*(*(self.0.as_ref().borrow()))))
                }
                // ============================================================
                /// with_mut
                pub fn with_mut<T, E>(
                    &self,
                    f: impl FnOnce(&mut dyn $base) -> StdResult<T, E>,
                ) -> StdResult<T, E>
                where
                    E: From<Error>,
                    dyn $base: Debug + EnableElicitFromSelf,
                {
                    f(&mut (*(*(self.0.as_ref().borrow_mut()))))
                }
            }
        }
    };
}
// ============================================================================
/// enable_elicit_from_self_delegate
#[macro_export]
macro_rules! enable_elicit_from_self_delegate {
    // ========================================================================
    ($base:ident, $elicit:ident) => {  // empty
        // --------------------------------------------------------------------
        fn elicit(&self) -> Option<$elicit> {
            None
        }
        // --------------------------------------------------------------------
        fn _weak_assign(&mut self,
                        _: ::std::rc::Weak<
                        ::std::cell::RefCell<Box<dyn $base>>>) {
        }
    };
    // ========================================================================
    ($base:ident, $elicit:ident, $field:ident) => {  // delegate to field
        // --------------------------------------------------------------------
        fn elicit(&self) -> Option<$elicit> {
            self.$field.elicit()
        }
        // --------------------------------------------------------------------
        fn _weak_assign(&mut self,
                        w: ::std::rc::Weak<
                        ::std::cell::RefCell<Box<dyn $base>>>) {
            self.$field._weak_assign(w)
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
#[allow(unreachable_pub)]
mod tests {
    // ////////////////////////////////////////////////////////////////////////
    // use  ===================================================================
    use super::super::Error;
    // type  ==================================================================
    type Result<T> = ::std::result::Result<T, Error>;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    elicit_define!(elicit_t0, T0);
    pub use self::elicit_t0::Elicit as Elicit_T0;
    pub use self::elicit_t0::EnableElicitFromSelf as EEFS_T0;
    pub use self::elicit_t0::EnableElicitFromSelfField as EEFS_Field_T0;
    // pub use self::elicit_t0::WeakElicit as WeakElicit_T0;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// trait T0
    pub trait T0: ::std::fmt::Debug + EEFS_T0 {
        /// get
        fn get(&self) -> i32;
        /// set
        fn set(&mut self, i: i32) -> ();
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct S0
    #[derive(Debug)]
    pub struct S0 {
        /// EEFS_
        _eefsf: EEFS_Field_T0,
        /// field
        field: i32,
    }
    // ========================================================================
    impl EEFS_T0 for S0 {
        enable_elicit_from_self_delegate!(T0, Elicit_T0, _eefsf);
    }
    // ========================================================================
    impl S0 {
        // ====================================================================
        /// new
        pub fn new(i: i32) -> Self {
            S0 {
                _eefsf: EEFS_Field_T0::default(),
                field: i,
            }
        }
    }
    // ========================================================================
    impl T0 for S0 {
        fn get(&self) -> i32 {
            self.field
        }
        fn set(&mut self, i: i32) {
            self.field = i;
        }
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct S1
    #[derive(Debug)]
    pub struct S1 {
        /// field
        field: i32,
    }
    // ========================================================================
    impl EEFS_T0 for S1 {
        enable_elicit_from_self_delegate!(T0, Elicit_T0);
    }
    // ========================================================================
    impl S1 {
        // ====================================================================
        /// new
        pub fn new(i: i32) -> Self {
            S1 { field: i }
        }
    }
    // ========================================================================
    impl T0 for S1 {
        fn get(&self) -> i32 {
            self.field
        }
        fn set(&mut self, i: i32) {
            self.field = i;
        }
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    #[test]
    fn elicit_with() {
        let vs = vec![Elicit_T0::new(S0::new(0)), Elicit_T0::new(S1::new(0))];
        for v in vs.iter() {
            assert!(
                v.with(|x: &dyn T0| -> Result<i32> { Ok(x.get()) }).unwrap()
                    == 0,
                "Elicit::with"
            );
            assert!(
                v.with_mut(|x: &mut dyn T0| -> Result<i32> {
                    x.set(10);
                    Ok(x.get())
                }).unwrap()
                    == 10,
                "Elicit::with_mut"
            );
        }
    }
}
