/* -*- mode:rust; coding:utf-8-unix; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/08
// @date 2016/05/08

// The MIT License (MIT)
//
// Copyright (c) <2016> hanepjiv <hanepjiv@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! elicit.rs
//!
//! # Examples
//!
//! ```
//! #[macro_use] extern crate elicit;
//!
//! elicit_define!(elicit_your_trait, YourTrait);
//! use self::elicit_your_trait::ElicitError
//!     as YourTraitElicitError;
//! use self::elicit_your_trait::ElicitResult
//!     as YourTraitElicitResult;
//! use self::elicit_your_trait::Elicit
//!     as YourTraitElicit;
//! use self::elicit_your_trait::EnableElicitFromSelf
//!     as YourTraitEnableElicitFromSelf;
//! use self::elicit_your_trait::EnableElicitFromSelfField
//!     as YourTraitEnableElicitFromSelfField;
//!
//! pub trait YourTrait: ::std::fmt::Debug + YourTraitEnableElicitFromSelf {
//!     fn your_function(&self) -> i32;
//! }
//!
//! #[derive( Debug, )]
//! pub struct YourStruct {
//!     _eefsf:        YourTraitEnableElicitFromSelfField,
//!     your_field:     i32,
//! }
//! impl YourTraitEnableElicitFromSelf for YourStruct {
//!     enable_elicit_from_self_impl_inner!(YourTrait, YourTraitElicit, _eefsf);
//! }
//! impl YourTrait for YourStruct {
//!     fn your_function(&self) -> i32 { self.your_field }
//! }
//!
//! #[derive( Debug, )]
//! pub struct YourStructUnuseEnableElicitFromSelf {
//!     your_field:     i32,
//! }
//! impl YourTraitEnableElicitFromSelf for YourStructUnuseEnableElicitFromSelf {
//!     enable_elicit_from_self_impl_inner!(YourTrait, YourTraitElicit);
//! }
//! impl YourTrait for YourStructUnuseEnableElicitFromSelf {
//!     fn your_function(&self) -> i32 { self.your_field }
//! }
//!
//! fn main() {
//!     let your0 = YourTraitElicit::new(YourStruct{
//!         _eefsf: YourTraitEnableElicitFromSelfField::default(),
//!         your_field: 0i32,
//!     });
//!     let your1 = YourTraitElicit::new(YourStructUnuseEnableElicitFromSelf{
//!         your_field: 1i32,
//!     });
//! }
//! ```

/* ////////////////////////////////////////////////////////////////////////// */
/* attribute  =============================================================== */
#![deny(missing_docs, dead_code, unused_imports, unused_variables)]
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// elicit_define!
#[macro_export]
macro_rules! elicit_define {
    ($modname:ident, $base:ident) => {
        /* ////////////////////////////////////////////////////////////////// */
        /* ================================================================== */
        pub mod $modname {
            //! $modname
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            use super::{ $base, };
            /* ============================================================== */
            use ::std::fmt::{ Debug, };
            use ::std::any::{ Any, };
            use ::std::sync::{ Arc, Weak,
                               RwLock, LockResult, TryLockResult, TryLockError,
                               RwLockReadGuard, RwLockWriteGuard, };
            use ::std::ops::{ Deref, DerefMut, };
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            /// struct Elicit
            #[derive( Debug, Clone, )]
            pub struct Elicit(Arc< RwLock< Box< $base > > >);
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            /// enum Error
            #[derive( Debug, Clone, )]
            pub enum ElicitError< E > {
                /// PoisonedRead
                PoisonedRead(Elicit),
                /// PoisonedWrite
                PoisonedWrite(Elicit),
                /// WouldBlock
                WouldBlock,
                /// Function
                Function(E),
            }
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            /// type ElicitResult
            pub type ElicitResult< R, E > = Result< R, ElicitError< E > >;
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            /// trait EnableElicitFromSelf
            pub trait EnableElicitFromSelf: ::std::fmt::Debug {
                /* ========================================================== */
                /// elicit_from_self
                fn elicit_from_self(&self) -> Option< Elicit >;
                /* ---------------------------------------------------------- */
                /// _weak_assign
                fn _weak_assign(&mut self,
                                weak: Weak< RwLock< Box< $base > > >) -> ();
            }
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            /// struct EnableElicitFromSelfField
            #[derive( Debug, Clone, )]
            pub struct EnableElicitFromSelfField {
                /// Weak
                _weak:  Option< Weak< RwLock< Box< $base > > > >,
            }
            /* ============================================================== */
            impl Default for EnableElicitFromSelfField {
                /* ========================================================== */
                fn default() -> Self { EnableElicitFromSelfField {
                    _weak:  None,
                } }
            }
            /* ============================================================== */
            impl EnableElicitFromSelf for EnableElicitFromSelfField {
                /* ========================================================== */
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
                /* ---------------------------------------------------------- */
                /// _weak_assign
                fn _weak_assign(&mut self,
                                weak: Weak< RwLock< Box< $base > > >) -> () {
                    self._weak = Some(weak)
                }
            }
            /* ////////////////////////////////////////////////////////////// */
            /* ============================================================== */
            impl Elicit {
                /* ========================================================== */
                /// new
                pub fn new< T >(val: T) -> Self
                    where T: Any + $base,
                          $base: Debug + EnableElicitFromSelf, {
                    let arc =
                        Arc::new(RwLock::new(Box::new(val) as Box< $base >));
                    arc.write().expect("Elicit::new").
                        _weak_assign(Arc::downgrade(&arc));
                    Elicit(arc)
                }
                /* ========================================================== */
                /// read
                    pub fn read(&self) ->
                    LockResult< RwLockReadGuard< Box< $base > > >
                    where $base: Debug + EnableElicitFromSelf, {
                        let &Elicit(ref inner) = self;
                        inner.read()
                    }
                /* ========================================================== */
                /// try_read
                    pub fn try_read(&self) ->
                    TryLockResult< RwLockReadGuard< Box< $base > > >
                    where $base: Debug + EnableElicitFromSelf, {
                        let &Elicit(ref inner) = self;
                        inner.try_read()
                    }
                /* ========================================================== */
                /// write
                    pub fn write(&self) ->
                    LockResult< RwLockWriteGuard< Box< $base > > > {
                        let &Elicit(ref inner) = self;
                        inner.write()
                    }
                /* ========================================================== */
                /// try_write
                pub fn try_write(&self) ->
                    TryLockResult< RwLockWriteGuard< Box< $base > > >
                    where $base: Debug + EnableElicitFromSelf, {
                    let &Elicit(ref inner) = self;
                    inner.try_write()
                }
                /* ========================================================== */
                /// with
                pub fn with< R, E, F, >(&self, f: F)
                                        -> ElicitResult< R, E >
                    where F: Fn(&$base) -> Result< R, E >,
                          $base: Debug + EnableElicitFromSelf, {
                    match self.read() {
                        Ok(ref x0) =>
                            f(& *(x0.deref().deref())).map_err(
                                |e| -> ElicitError< E > {
                                    ElicitError::Function(e)
                                }),
                        Err(_) => Err(ElicitError::PoisonedRead(self.clone())),
                    }
                }
                /* ========================================================== */
                /// try_with
                pub fn try_with< R, E, F, >(&self, f: F)
                                            -> ElicitResult< R, E >
                    where F: Fn(&$base) -> Result< R, E >,
                          $base: Debug + EnableElicitFromSelf, {
                    match self.try_read() {
                        Ok(ref x0) =>
                            f(& *(x0.deref().deref())).map_err(
                                |e| -> ElicitError< E > {
                                    ElicitError::Function(e)
                                }),
                        Err(e0) => match e0 {
                            TryLockError::Poisoned(_)   =>
                                Err(ElicitError::PoisonedRead(self.clone())),
                            TryLockError::WouldBlock    =>
                                Err(ElicitError::WouldBlock),
                        },
                    }
                }
                /* ========================================================== */
                /// with_mut
                pub fn with_mut< R, E, F, >(&self, f: F)
                                            -> ElicitResult< R, E >
                    where F: Fn(&mut $base) -> Result< R, E >,
                          $base: Debug + EnableElicitFromSelf, {
                    match self.write() {
                        Ok(ref mut x0) =>
                            f(&mut *(x0.deref_mut().deref_mut())).map_err(
                                |e| -> ElicitError< E > {
                                    ElicitError::Function(e)
                                }),
                        Err(_) => Err(ElicitError::PoisonedWrite(self.clone())),
                    }
                }
                /* ========================================================== */
                /// try_with_mut
                pub fn try_with_mut< R, E, F, >(&self, f: F)
                                                -> ElicitResult< R, E >
                    where F: Fn(& mut $base) -> Result< R, E >,
                          $base: Debug + EnableElicitFromSelf, {
                    match self.try_write() {
                        Ok(ref mut x0) =>
                            f(&mut *(x0.deref_mut().deref_mut())).map_err(
                                |e| -> ElicitError< E > {
                                    ElicitError::Function(e)
                                }),
                        Err(e0) => match e0 {
                            TryLockError::Poisoned(_)   =>
                                Err(ElicitError::PoisonedWrite(self.clone())),
                            TryLockError::WouldBlock    =>
                                Err(ElicitError::WouldBlock),
                        },
                    }
                }
            }
        }
    };
}
/* ========================================================================== */
/// enable_elicit_from_self_impl_inner
#[macro_export]
macro_rules! enable_elicit_from_self_impl_inner {
    /* ====================================================================== */
    ($base:ident, $elicit:ident)                => {  // empty
        /* ------------------------------------------------------------------ */
        fn elicit_from_self(&self) -> Option< $elicit > {
            None
        }
        /* ------------------------------------------------------------------ */
        fn _weak_assign(&mut self,
                        _: ::std::sync::Weak< ::std::sync::RwLock<
                        Box< $base > > >) -> () {
        }
    };
    /* ====================================================================== */
    ($base:ident, $elicit:ident, $field:ident)  => {  // delegate to field
        /* ------------------------------------------------------------------ */
        fn elicit_from_self(&self) -> Option< $elicit > {
            self.$field.elicit_from_self()
        }
        /* ------------------------------------------------------------------ */
        fn _weak_assign(&mut self,
                        weak: ::std::sync::Weak< ::std::sync::RwLock<
                        Box< $base > > >) -> () {
            self.$field._weak_assign(weak)
        }
    };
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
#[cfg(test)]
mod tests {
    /* ////////////////////////////////////////////////////////////////////// */
    /* ====================================================================== */
    elicit_define!(elicit_t0, T0);
    // use self::elicit_t0::ElicitError               as ElicitErrorT0;
    // use self::elicit_t0::ElicitResult              as ElicitResultT0;
    use self::elicit_t0::Elicit                    as ElicitT0;
    use self::elicit_t0::EnableElicitFromSelf      as EnableElicitFromSelfT0;
    use self::elicit_t0::EnableElicitFromSelfField
        as EnableElicitFromSelfFieldT0;
    /* ////////////////////////////////////////////////////////////////////// */
    /* ====================================================================== */
    /// trait T0
    pub trait T0: ::std::fmt::Debug + EnableElicitFromSelfT0 {
        /// get
        fn get(&self) -> i32;
        /// set
        fn set(&mut self, i: i32) -> ();
    }
    /* ////////////////////////////////////////////////////////////////////// */
    /* ====================================================================== */
    /// struct S0
    #[derive( Debug, )]
    pub struct S0 {
        /// EnableElicitFromSelf
        _eefsf:         EnableElicitFromSelfFieldT0,
        /// field
        field:          i32,
    }
    /* ====================================================================== */
    impl EnableElicitFromSelfT0 for S0 {
        enable_elicit_from_self_impl_inner!(T0, ElicitT0, _eefsf);
    }
    /* ====================================================================== */
    impl S0 {
        /* ================================================================== */
        /// new
        pub fn new(i: i32) -> Self { S0 {
            _eefsf:     EnableElicitFromSelfFieldT0::default(),
            field:      i,
        } }
    }
    /* ====================================================================== */
    impl T0 for S0 {
        fn get(&self) -> i32 { self.field }
        fn set(&mut self, i: i32) { self.field = i; }
    }
    /* ////////////////////////////////////////////////////////////////////// */
    /* ====================================================================== */
    /// struct S1
    #[derive( Debug, )]
    pub struct S1 {
        /// field
        field:          i32,
    }
    /* ====================================================================== */
    impl EnableElicitFromSelfT0 for S1 {
        enable_elicit_from_self_impl_inner!(T0, ElicitT0);
    }
    /* ====================================================================== */
    impl S1 {
        /* ================================================================== */
        /// new
        pub fn new(i: i32) -> Self { S1 {
            field:      i,
        } }
    }
    /* ====================================================================== */
    impl T0 for S1 {
        fn get(&self) -> i32 { self.field }
        fn set(&mut self, i: i32) { self.field = i; }
    }
    /* ////////////////////////////////////////////////////////////////////// */
    /* ====================================================================== */
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
        for v in vs.iter() {
            assert!(v.try_with(|x: &T0| -> Result<i32, i32> {
                Ok(x.get())
            }).unwrap() == 10, "Elicit::try_with");
            assert!(v.try_with_mut(|x: &mut T0| -> Result<i32, i32> {
                x.set(20);
                Ok(x.get())
            }).unwrap() == 20, "Elicit::try_with_mut");
        }
    }
}
