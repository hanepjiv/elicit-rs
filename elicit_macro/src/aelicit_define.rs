// -*- mode:rust; coding:utf-8-unix; -*-

//! aelicit_define.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2024/04/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::*;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// fn expand
pub(crate) fn expand(
    mod_ident: Ident,
    item: ItemTrait,
) -> Result<TokenStream2> {
    let define = quote_define(mod_ident, &item)?;
    let mut ret = item.into_token_stream();
    ret.extend(define);
    Ok(ret)
}
// ----------------------------------------------------------------------------
/// fn quote_define
fn quote_define(mod_ident: Ident, item: &ItemTrait) -> Result<TokenStream2> {
    let inner = quote_inner(&item.ident)?;
    Ok(quote! {
        #[allow(box_pointers)]
        mod #mod_ident {
            mod _inner { #inner }

            mod _common {
                pub use super::_inner::{
                    Aelicit, AelicitBase, AelicitFromSelf,
                };
            }

            /// mod autho
            pub mod author {
                pub use super::_common::*;
                pub use super::_inner::{
                    WeakAssign,
                    AelicitFromSelfField
                };
            }

            /// mod user
            pub mod user {
                pub use super::_common::*;
                pub use super::_inner::WeakAelicit;
            }
        }
    })
}
// ----------------------------------------------------------------------------
/// fn quote_inner
fn quote_inner(a_orig: &Ident) -> Result<TokenStream2> {
    let orig = quote! { super::super::#a_orig };
    Ok(quote! {
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        use std::{
            convert::From,
            fmt::Debug,
            result::Result as StdResult,
            sync::{
                OnceLock, Arc, LockResult,
                RwLock, RwLockReadGuard, RwLockWriteGuard,
                TryLockError, TryLockResult, Weak,
            },
        };
        pub use elicit::{ Result as ElicitResult, Error as ElicitError };
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// trait AelicitBase
        pub trait AelicitBase:
            'static + Debug + #orig + AelicitFromSelf + WeakAssign
        {
        }
        // ===============================================================
        impl<T: 'static + Debug + #orig + AelicitFromSelf + WeakAssign>
            AelicitBase for T
        {
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct Aelicit
        #[derive(Debug, Clone)]
        pub struct Aelicit(Arc<RwLock<Box<dyn AelicitBase>>>);
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct WeakAelicit
        #[derive(Debug, Clone)]
        pub struct WeakAelicit(Weak<RwLock<Box<dyn AelicitBase>>>);
        // ================================================================
        impl WeakAelicit {
            // ============================================================
            /// fn upgrade
            pub fn upgrade(&self) -> Option<Aelicit> {
                self.0.upgrade().map(Aelicit)
            }
        }
        // ================================================================
        impl From<Aelicit> for WeakAelicit {
            fn from(x: Aelicit) -> WeakAelicit {
                x.weak()
            }
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// trait AelicitFromSelf
        pub trait AelicitFromSelf {
            /// aelicit_from_self
            fn aelicit_from_self(&self) -> Option<Aelicit>;
        }
        // ================================================================
        /// trait WeakAssign
        pub trait WeakAssign {
            /// _weak_assign
            fn _weak_assign(
                &mut self,
                weak: Weak<RwLock<Box<dyn AelicitBase>>>,
            ) -> ElicitResult<()>;
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct AelicitFromSelfField
        #[derive(Debug, Clone, Default)]
        pub struct AelicitFromSelfField {
            /// _weak
            _weak: OnceLock<Weak<RwLock<Box<dyn AelicitBase>>>>,
        }
        // ================================================================
        impl AelicitFromSelf for AelicitFromSelfField {
            fn aelicit_from_self(&self) -> Option<Aelicit> {
                self._weak.get()?.upgrade().map(Aelicit)
            }
        }
        // ================================================================
        impl WeakAssign for AelicitFromSelfField {
            fn _weak_assign(
                &mut self,
                weak: Weak<RwLock<Box<dyn AelicitBase>>>,
            ) -> ElicitResult<()> {
                self._weak.set(weak).map_err(
                    |_| ElicitError::WeakAlreadyExists)
            }
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        impl Aelicit {
            // ============================================================
            /// new
            #[allow(trivial_casts)]
            pub fn new<T>(val: T) -> ElicitResult<Self>
            where
                T: AelicitBase,
            {
                let r = Arc::new(RwLock::new(
                    Box::new(val) as Box<dyn AelicitBase>
                ));
                let weak = Arc::<_>::downgrade(&r);
                let ret = Aelicit(r);
                let _ = ret.with_mut(|x| x._weak_assign(weak))?;
                Ok(ret)

            }
            // ============================================================
            /// weak
            pub fn weak(&self) -> WeakAelicit {
                WeakAelicit(Arc::downgrade(&self.0))
            }
            // ============================================================
            /// read
            pub fn read(
                &self,
            ) -> LockResult<RwLockReadGuard<'_, Box<dyn AelicitBase>>>
            {
                self.0.read()
            }
            // ============================================================
            /// try_read
            pub fn try_read(
                &self,
            ) -> TryLockResult<RwLockReadGuard<'_, Box<dyn AelicitBase>>>
            {
                self.0.try_read()
            }
            // ============================================================
            /// write
            pub fn write(
                &self,
            ) -> LockResult<RwLockWriteGuard<'_, Box<dyn AelicitBase>>>
            {
                self.0.write()
            }
            // ============================================================
            /// try_write
            pub fn try_write(
                &self,
            ) -> TryLockResult<RwLockWriteGuard<'_, Box<dyn AelicitBase>>>
            where
                dyn AelicitBase: Debug + AelicitFromSelf,
            {
                self.0.try_write()
            }
            // ============================================================
            /// with
            pub fn with<T, E>(
                &self,
                f: impl FnOnce(&dyn AelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.read().map_or_else(
                    |_| Err(E::from(ElicitError::Poisoned)),
                    |x| f(x.as_ref())
                )
            }
            // ============================================================
            /// try_with
            pub fn try_with<T, E>(
                &self,
                f: impl FnOnce(&dyn AelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.try_read().map_or_else(
                    |x| match x {
                        TryLockError::Poisoned(_) => {
                            Err(E::from(ElicitError::Poisoned))
                        }
                        TryLockError::WouldBlock => {
                            Err(E::from(ElicitError::WouldBlock))
                        }
                    },
                    |x| f(x.as_ref())
                )
            }
            // ============================================================
            /// with_mut
            pub fn with_mut<T, E>(
                &self,
                f: impl FnOnce(&mut dyn AelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.write().map_or_else(
                    |_| Err(E::from(ElicitError::Poisoned)),
                    |mut x| f(x.as_mut())
                )
            }
            // ============================================================
            /// try_with_mut
            pub fn try_with_mut<T, E>(
                &self,
                f: impl FnOnce(&mut dyn AelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.try_write().map_or_else(
                    |x| match x {
                        TryLockError::Poisoned(_) => {
                            Err(E::from(ElicitError::Poisoned))
                        }
                        TryLockError::WouldBlock => {
                            Err(E::from(ElicitError::WouldBlock))
                        }
                    },
                    |mut x| f(x.as_mut())
                )
            }
        }
    })
}
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse2;
    // ========================================================================
    #[test]
    fn test_00() {
        assert!(expand(
            parse2(quote!(mod_ident)).expect("parse attr"),
            parse2(quote!(
                trait Orig {}
            ))
                .expect("parse item")
        )
                .is_ok());
    }
}
