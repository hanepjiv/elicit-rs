// -*- mode:rust; coding:utf-8-unix; -*-

//! elicit.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/08/18
//  @date 2016/12/27

//! # Examples
//!
//! ```
//! #[macro_use] extern crate elicit;
//!
//! elicit_define!(elicit_my_trait, MyTrait);
//! use self::elicit_my_trait::ElicitError
//!     as MyTraitElicitError;
//! use self::elicit_my_trait::ElicitResult
//!     as MyTraitElicitResult;
//! use self::elicit_my_trait::Elicit
//!     as MyTraitElicit;
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
//! pub struct MyStruct {
//!     _eefsf:        MyTraitEnableElicitFromSelfField,
//!     my_field:     i32,
//! }
//! impl MyTraitEnableElicitFromSelf for MyStruct {
//!     enable_elicit_from_self_impl_inner!(MyTrait, MyTraitElicit, _eefsf);
//! }
//! impl MyTrait for MyStruct {
//!     fn my_function(&self) -> i32 { self.my_field }
//! }
//!
//! #[derive( Debug, )]
//! pub struct MyStructUnuseEnableElicitFromSelf {
//!     my_field:     i32,
//! }
//! impl MyTraitEnableElicitFromSelf for MyStructUnuseEnableElicitFromSelf {
//!     enable_elicit_from_self_impl_inner!(MyTrait, MyTraitElicit);
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
        pub mod $modname {
            //! $modname
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            use super::$base;
            // ================================================================
            use ::std::fmt::{ Debug, Display, };
            use ::std::any::Any;
            use ::std::cell::RefCell;
            use ::std::rc::{ Rc, Weak, };
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct Elicit
            #[derive( Debug, Clone, )]
            pub struct Elicit(Rc< RefCell< Box< $base > > >);
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// enum Error
            #[derive( Debug, Clone, )]
            pub enum ElicitError< E > {
                /// Function
                Function(E),
            }
            // ================================================================
            impl < E > Display for ElicitError< E >
                where E:        Display,                {
                // ============================================================
                fn fmt(&self, f: &mut ::std::fmt::Formatter)
                       -> ::std::fmt::Result { match *self {
                    ElicitError::Function(ref e)        => e.fmt(f),
                } }
            }
            // ================================================================
            impl < E > ::std::error::Error for ElicitError< E >
                where E:        ::std::error::Error,    {
                // ============================================================
                fn description(&self) -> &str { match *self {
                    ElicitError::Function(ref e)        => e.description(),
                } }
                // ============================================================
                fn cause(&self) -> Option<&::std::error::Error> { match *self {
                    ElicitError::Function(ref e)        => Some(e),
                } }
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// type ElicitResult
            pub type ElicitResult< R, E > = Result< R, ElicitError< E > >;
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// trait EnableElicitFromSelf
            pub trait EnableElicitFromSelf: Debug {
                // ============================================================
                /// elicit_from_self
                fn elicit_from_self(&self) -> Option< Elicit >;
                // ------------------------------------------------------------
                /// _weak_assign
                fn _weak_assign(&mut self,
                                weak: Weak< RefCell< Box< $base > > >) -> ();
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct EnableElicitFromSelfField
            #[derive( Debug, Clone, Default, )]
            pub struct EnableElicitFromSelfField {
                /// Weak
                _weak:  Option< Weak< RefCell< Box< $base > > > >,
            }
            // ================================================================
            impl EnableElicitFromSelf for EnableElicitFromSelfField {
                // ============================================================
                /// elicit_from_self
                fn elicit_from_self(&self) -> Option< Elicit > {
                    match self._weak {
                        None            => None,
                        Some(ref x)     => {
                            Some(Elicit(x.upgrade().
                                        expect("elicit_from_self")))
                        }
                    }
                }
                // ------------------------------------------------------------
                /// _weak_assign
                fn _weak_assign(&mut self,
                                weak: Weak< RefCell< Box< $base > > >) -> () {
                    self._weak = Some(weak)
                }
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            impl Elicit {
                // ============================================================
                /// new
                pub fn new< T >(val: T) -> Self
                    where T:            Any + $base,
                          $base:        Debug + EnableElicitFromSelf,   {
                    let rc =
                        Rc::new(RefCell::new(Box::new(val) as Box<$base>));
                    rc.as_ref().borrow_mut()._weak_assign(Rc::downgrade(&rc));
                    Elicit(rc)
                }
                // ============================================================
                /// with
                pub fn with< R, E, F, >(&self, f: F)
                                        -> ElicitResult< R, E >
                    where F:            FnOnce(&$base) -> Result< R, E >,
                          $base:        Debug + EnableElicitFromSelf,   {
                    let &Elicit(ref inner) = self;
                    f(&(*(*(inner.as_ref().borrow())))).map_err(
                        |e| -> ElicitError< E > {
                            ElicitError::Function(e)
                        })
                }
                // ============================================================
                /// with_mut
                pub fn with_mut< R, E, F, >(&self, f: F)
                                            -> ElicitResult< R, E >
                    where F:            FnOnce(&mut $base) -> Result< R, E >,
                          $base:        Debug + EnableElicitFromSelf,   {
                    let &Elicit(ref inner) = self;
                    f(&mut(*(*(inner.as_ref().borrow_mut())))).map_err(
                        |e| -> ElicitError< E > {
                            ElicitError::Function(e)
                        })
                }
            }
        }
    };
}
// ============================================================================
/// enable_elicit_from_self_impl_inner
#[macro_export]
macro_rules! enable_elicit_from_self_impl_inner {
    // ========================================================================
    ($base:ident, $elicit:ident)                => {  // empty
        // --------------------------------------------------------------------
        fn elicit_from_self(&self) -> Option< $elicit > {
            None
        }
        // --------------------------------------------------------------------
        fn _weak_assign(&mut self,
                        _: ::std::rc::Weak<::std::cell::RefCell<Box<$base>>>)
                        -> () {
        }
    };
    // ========================================================================
    ($base:ident, $elicit:ident, $field:ident)  => {  // delegate to field
        // --------------------------------------------------------------------
        fn elicit_from_self(&self) -> Option< $elicit > {
            self.$field.elicit_from_self()
        }
        // --------------------------------------------------------------------
        fn _weak_assign(&mut self,
                        w: ::std::rc::Weak<::std::cell::RefCell<Box<$base>>>)
                        -> () {
            self.$field._weak_assign(w)
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    elicit_define!(elicit_t0, T0);
    // use self::elicit_t0::ElicitError               as ElicitErrorT0;
    // use self::elicit_t0::ElicitResult              as ElicitResultT0;
    use self::elicit_t0::Elicit                    as ElicitT0;
    use self::elicit_t0::EnableElicitFromSelf      as EnableElicitFromSelfT0;
    use self::elicit_t0::EnableElicitFromSelfField
        as EnableElicitFromSelfFieldT0;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// trait T0
    pub trait T0: ::std::fmt::Debug + EnableElicitFromSelfT0 {
        /// get
        fn get(&self) -> i32;
        /// set
        fn set(&mut self, i: i32) -> ();
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct S0
    #[derive( Debug, )]
    pub struct S0 {
        /// EnableElicitFromSelf
        _eefsf:         EnableElicitFromSelfFieldT0,
        /// field
        field:          i32,
    }
    // ========================================================================
    impl EnableElicitFromSelfT0 for S0 {
        enable_elicit_from_self_impl_inner!(T0, ElicitT0, _eefsf);
    }
    // ========================================================================
    impl S0 {
        // ====================================================================
        /// new
        pub fn new(i: i32) -> Self { S0 {
            _eefsf:     EnableElicitFromSelfFieldT0::default(),
            field:      i,
        } }
    }
    // ========================================================================
    impl T0 for S0 {
        fn get(&self) -> i32 { self.field }
        fn set(&mut self, i: i32) { self.field = i; }
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// struct S1
    #[derive( Debug, )]
    pub struct S1 {
        /// field
        field:          i32,
    }
    // ========================================================================
    impl EnableElicitFromSelfT0 for S1 {
        enable_elicit_from_self_impl_inner!(T0, ElicitT0);
    }
    // ========================================================================
    impl S1 {
        // ====================================================================
        /// new
        pub fn new(i: i32) -> Self { S1 {
            field:      i,
        } }
    }
    // ========================================================================
    impl T0 for S1 {
        fn get(&self) -> i32 { self.field }
        fn set(&mut self, i: i32) { self.field = i; }
    }
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    #[test]
    fn elicit_with() {
        let vs = vec![
            ElicitT0::new(S0::new(0)),
            ElicitT0::new(S1::new(0)),
        ];
        for v in vs.iter() {
            assert!(v.with(|x: &T0| -> Result<i32, i32> {
                Ok(x.get())
            }).unwrap() ==  0, "Elicit::with");
            assert!(v.with_mut(|x: &mut T0| -> Result<i32, i32> {
                x.set(10);
                Ok(x.get())
            }).unwrap() == 10, "Elicit::with_mut");
        }
    }
}
