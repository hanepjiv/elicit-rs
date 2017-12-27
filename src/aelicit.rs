// -*- mode:rust; coding:utf-8-unix; -*-

//! aelicit.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/08/18
//  @date 2017/02/24

//! # Examples
//!
//! ```
//! #[macro_use] extern crate elicit;
//!
//! aelicit_define!(aelicit_my_trait, MyTrait);
//! pub use self::aelicit_my_trait::Aelicit
//!     as MyTraitAelicit;
//! pub use self::aelicit_my_trait::EnableAelicitFromSelf
//!     as MyTraitEnableAelicitFromSelf;
//! pub use self::aelicit_my_trait::EnableAelicitFromSelfField
//!     as MyTraitEnableAelicitFromSelfField;
//!
//! pub trait MyTrait: ::std::fmt::Debug + MyTraitEnableAelicitFromSelf {
//!     fn my_function(&self) -> i32;
//! }
//!
//! #[derive( Debug, )]
//! pub struct MyStruct {
//!     _eafsf:        MyTraitEnableAelicitFromSelfField,
//!     my_field:     i32,
//! }
//! impl MyTraitEnableAelicitFromSelf for MyStruct {
//!     enable_aelicit_from_self_delegate!(MyTrait, MyTraitAelicit, _eafsf);
//! }
//! impl MyTrait for MyStruct {
//!     fn my_function(&self) -> i32 { self.my_field }
//! }
//!
//! #[derive( Debug, )]
//! pub struct MyStructUnuseEnableAelicitFromSelf {
//!     my_field:     i32,
//! }
//! impl MyTraitEnableAelicitFromSelf for MyStructUnuseEnableAelicitFromSelf {
//!     enable_aelicit_from_self_delegate!(MyTrait, MyTraitAelicit);
//! }
//! impl MyTrait for MyStructUnuseEnableAelicitFromSelf {
//!     fn my_function(&self) -> i32 { self.my_field }
//! }
//!
//! fn main() {
//!     let my0 = MyTraitAelicit::new(MyStruct{
//!         _eafsf: MyTraitEnableAelicitFromSelfField::default(),
//!         my_field: 0i32,
//!     });
//!     let my1 = MyTraitAelicit::new(MyStructUnuseEnableAelicitFromSelf{
//!         my_field: 1i32,
//!     });
//! }
//! ```

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// aelicit_define!
#[macro_export]
macro_rules! aelicit_define {
    ($modname:ident, $base:ident) => {
        // ////////////////////////////////////////////////////////////////////
        // ====================================================================
        pub mod $modname {
            //! $modname
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            use super::$base;
            // ================================================================
            use ::std::any::Any;
            use ::std::convert::From;
            use ::std::fmt::Debug;
            use ::std::result::Result as StdResult;
            use ::std::sync::{ Arc, Weak,
                               RwLock, LockResult, TryLockResult, TryLockError,
                               RwLockReadGuard, RwLockWriteGuard };
            use $crate::Error;
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct Aelicit
            #[derive( Debug, Clone, )]
            pub struct Aelicit(Arc<RwLock<Box<$base>>>);
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// trait EnableAelicitFromSelf
            pub trait EnableAelicitFromSelf: Debug {
                // ============================================================
                /// aelicit_from_self
                fn aelicit_from_self(&self) -> Option<Aelicit>;
                // ------------------------------------------------------------
                /// _weak_assign
                fn _weak_assign(&mut self, weak: Weak<RwLock<Box<$base>>>);
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            /// struct EnableAelicitFromSelfField
            #[derive( Debug, Clone, Default, )]
            pub struct EnableAelicitFromSelfField {
                /// Weak
                _weak:  Option<Weak<RwLock<Box<$base>>>>,
            }
            // ================================================================
            impl EnableAelicitFromSelf for EnableAelicitFromSelfField {
                // ============================================================
                /// aelicit_from_self
                fn aelicit_from_self(&self) -> Option<Aelicit> {
                    if let Some(ref x) = self._weak {
                        Some(Aelicit(x.upgrade().expect("aelicit_from_self")))
                    } else {
                        None
                    }
                }
                // ------------------------------------------------------------
                /// _weak_assign
                fn _weak_assign(&mut self, weak: Weak<RwLock<Box<$base>>>) {
                    self._weak = Some(weak)
                }
            }
            // ////////////////////////////////////////////////////////////////
            // ================================================================
            impl Aelicit {
                // ============================================================
                /// new
                pub fn new<T>(val: T) -> Self
                    where T:            Any + $base,
                          $base:        Debug + EnableAelicitFromSelf,  {
                    let arc =
                        Arc::new(RwLock::new(Box::new(val) as Box<$base>));
                    arc.write().expect("Aelicit::new").
                        _weak_assign(Arc::downgrade(&arc));
                    Aelicit(arc)
                }
                // ============================================================
                /// read
                pub fn read(&self) ->
                    LockResult<RwLockReadGuard<Box<$base>>>
                    where $base:        Debug + EnableAelicitFromSelf,  {
                    self.0.read()
                }
                // ============================================================
                /// try_read
                pub fn try_read(&self) ->
                    TryLockResult<RwLockReadGuard<Box<$base>>>
                    where $base:        Debug + EnableAelicitFromSelf,  {
                    self.0.try_read()
                }
                // ============================================================
                /// write
                pub fn write(&self) ->
                    LockResult<RwLockWriteGuard<Box<$base>>> {
                        self.0.write()
                    }
                // ============================================================
                /// try_write
                pub fn try_write(&self) ->
                    TryLockResult<RwLockWriteGuard<Box<$base>>>
                    where $base:        Debug + EnableAelicitFromSelf,  {
                    self.0.try_write()
                }
                // ============================================================
                /// with
                pub fn with<T, E, F>(&self, f: F) -> StdResult<T, E>
                    where E:            From<Error>,
                          F:            FnOnce(&$base) -> StdResult<T, E>,
                          $base:        Debug + EnableAelicitFromSelf,  {
                    match self.read() {
                        Err(e)  => {
                            debug!("::elicit::Aelicit::with: {}", e);
                            Err(E::from(Error::PoisonedRead))
                        },
                        Ok(x)   => f(x.as_ref())
                    }
                }
                // ============================================================
                /// try_with
                pub fn try_with<T, E, F>(&self, f: F) -> StdResult<T, E>
                    where E:            From<Error>,
                          F:            FnOnce(&$base) -> StdResult<T, E>,
                          $base:        Debug + EnableAelicitFromSelf,  {
                    match self.try_read() {
                        Err(e)          => {
                            debug!("::elicit::Aelicit::try_with: {}", e);
                            match e {
                                TryLockError::Poisoned(_)   =>
                                    Err(E::from(Error::PoisonedRead)),
                                TryLockError::WouldBlock    =>
                                    Err(E::from(Error::WouldBlock)),
                            }
                        },
                        Ok(x)           => { f(x.as_ref()) },
                    }
                }
                // ============================================================
                /// with_mut
                pub fn with_mut<T, E, F>(&self, f: F) -> StdResult<T, E>
                    where E:            From<Error>,
                          F:            FnOnce(&mut $base) -> StdResult<T, E>,
                          $base:        Debug + EnableAelicitFromSelf,  {
                    match self.write() {
                        Err(e) => {
                            debug!("::elicit::Aelicit::with_mut: {}", e);
                            Err(E::from(Error::PoisonedWrite))
                        },
                        Ok(mut x) => f(&mut *(x.as_mut())),
                    }
                }
                // ============================================================
                /// try_with_mut
                pub fn try_with_mut<T, E, F>(&self, f: F) -> StdResult<T, E>
                    where E:            From<Error>,
                          F:            FnOnce(& mut $base) -> StdResult<T, E>,
                          $base:        Debug + EnableAelicitFromSelf,  {
                    match self.try_write() {
                        Err(e)          =>  {
                            debug!("::elicit::Aelicit::try_with_mut: {}", e);
                            match e {
                                TryLockError::Poisoned(_)   =>
                                    Err(E::from(Error::PoisonedWrite)),
                                TryLockError::WouldBlock    =>
                                    Err(E::from(Error::WouldBlock)),
                            }
                        },
                        Ok(mut x)       => f(&mut *(x.as_mut())),
                    }
                }
            }
        }
    };
}
// ============================================================================
/// enable_aelicit_from_self_delegate
#[macro_export]
macro_rules! enable_aelicit_from_self_delegate {
    // ========================================================================
    ($base:ident, $aelicit:ident)                => {  // empty
        // --------------------------------------------------------------------
        fn aelicit_from_self(&self) -> Option<$aelicit> {
            None
        }
        // --------------------------------------------------------------------
        fn _weak_assign(&mut self,
                        _: ::std::sync::Weak<::std::sync::RwLock<Box<$base>>>){
        }
    };
    // ========================================================================
    ($base:ident, $aelicit:ident, $field:ident)  => {  // delegate to field
        // --------------------------------------------------------------------
        fn aelicit_from_self(&self) -> Option<$aelicit> {
            self.$field.aelicit_from_self()
        }
        // --------------------------------------------------------------------
        fn _weak_assign(&mut self,
                        w: ::std::sync::Weak<::std::sync::RwLock<Box<$base>>>){
            self.$field._weak_assign(w)
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // ////////////////////////////////////////////////////////////////////////
    // use  ===================================================================
    use super::super::Error;
    // type  ==================================================================
    type Result<T> = ::std::result::Result<T, Error>;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    aelicit_define!(aelicit_t0, T0);
    pub use self::aelicit_t0::Aelicit as AelicitT0;
    pub use self::aelicit_t0::EnableAelicitFromSelf as EAFS_T0;
    pub use self::aelicit_t0::EnableAelicitFromSelfField as EAFS_Field_T0;
    // ////////////////////////////////////////////////////////////////////////
    // ========================================================================
    /// trait T0
    pub trait T0: ::std::fmt::Debug + EAFS_T0 {
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
        /// EnableAelicitFromSelf
        _eafsf: EAFS_Field_T0,
        /// field
        field: i32,
    }
    // ========================================================================
    impl EAFS_T0 for S0 {
        enable_aelicit_from_self_delegate!(T0, AelicitT0, _eafsf);
    }
    // ========================================================================
    impl S0 {
        // ====================================================================
        /// new
        pub fn new(i: i32) -> Self {
            S0 {
                _eafsf: EAFS_Field_T0::default(),
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
    impl EAFS_T0 for S1 {
        enable_aelicit_from_self_delegate!(T0, AelicitT0);
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
    fn aelicit_with() {
        let vs = vec![AelicitT0::new(S0::new(0)), AelicitT0::new(S1::new(0))];
        for v in vs.iter() {
            assert!(
                v.with(|x: &T0| -> Result<i32> { Ok(x.get()) }).unwrap() == 0,
                "Aelicit::with"
            );
            assert!(
                v.with_mut(|x: &mut T0| -> Result<i32> {
                    x.set(10);
                    Ok(x.get())
                }).unwrap() == 10,
                "Aelicit::with_mut"
            );
        }
        for v in vs.iter() {
            assert!(
                v.try_with(|x: &T0| -> Result<i32> { Ok(x.get()) }).unwrap()
                    == 10,
                "Aelicit::try_with"
            );
            assert!(
                v.try_with_mut(|x: &mut T0| -> Result<i32> {
                    x.set(20);
                    Ok(x.get())
                }).unwrap() == 20,
                "Aelicit::try_with_mut"
            );
        }
    }
}
