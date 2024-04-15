// -*- mode:rust; coding:utf-8-unix; -*-

//! melicit_define.rs

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
                    Melicit, MelicitBase, MelicitFromSelf,
                };
            }

            /// mod author
            pub mod author {
                pub use super::_common::*;
                pub use super::_inner::{
                    WeakAssign,
                    MelicitFromSelfField
                };
            }

            /// mod user
            pub mod user {
                pub use super::_common::*;
                pub use super::_inner::WeakMelicit;
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
                OnceLock, Arc, LockResult, Mutex, MutexGuard,
                TryLockError, TryLockResult, Weak, PoisonError
            },
        };
        pub use elicit::{ Result as ElicitResult, Error as ElicitError };
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// trait MelicitBase
        pub trait MelicitBase:
        'static + Debug + #orig + MelicitFromSelf + WeakAssign
        {
        }
        impl<T: 'static + Debug + #orig + MelicitFromSelf + WeakAssign>
            MelicitBase for T
        {
        }
        // ================================================================
        /// struct Melicit
        #[derive(Debug, Clone)]
        pub struct Melicit(Arc<Mutex<Box<dyn MelicitBase>>>);
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct WeakMelicit
        #[derive(Debug, Clone)]
        pub struct WeakMelicit(Weak<Mutex<Box<dyn MelicitBase>>>);
        // ================================================================
        impl WeakMelicit {
            // ============================================================
            /// fn upgrade
            pub fn upgrade(&self) -> Option<Melicit> {
                self.0.upgrade().map(Melicit)
            }
        }
        // ================================================================
        impl From<Melicit> for WeakMelicit {
            fn from(x: Melicit) -> WeakMelicit {
                x.weak()
            }
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// trait MelicitFromSelf
        pub trait MelicitFromSelf {
            /// melicit_from_self
            fn melicit_from_self(&self) -> Option<Melicit>;
        }
        // ================================================================
        /// trait WeakAssign
        pub trait WeakAssign {
            /// _weak_assign
            fn _weak_assign(
                &mut self,
                weak: Weak<Mutex<Box<dyn MelicitBase>>>,
            ) -> ElicitResult<()>;
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct MelicitFromSelfField
        #[derive(Debug, Clone, Default)]
        pub struct MelicitFromSelfField {
            /// _weak
            _weak: OnceLock<Weak<Mutex<Box<dyn MelicitBase>>>>,
        }
        // ================================================================
        impl MelicitFromSelf for MelicitFromSelfField {
            fn melicit_from_self(&self) -> Option<Melicit> {
                self._weak.get()?.upgrade().map(Melicit)
            }
        }
        // ================================================================
        impl WeakAssign for MelicitFromSelfField {
            fn _weak_assign(
                &mut self,
                weak: Weak<Mutex<Box<dyn MelicitBase>>>,
            ) -> ElicitResult<()> {
                self._weak.set(weak).map_err(
                    |_| ElicitError::WeakAlreadyExists)
            }
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        impl Melicit {
            // ============================================================
            /// new
            #[allow(trivial_casts)]
            pub fn new<T>(val: T) -> Self
            where
                T: MelicitBase,
            {
                let arc = Arc::new(Mutex::new(
                    Box::new(val) as Box<dyn MelicitBase>
                ));
                arc.lock()
                    .expect("Melicit::new: Mutex poisoned.")
                    ._weak_assign(Arc::downgrade(&arc))
                    .expect("Melicit::new: _weak already exists.");
                Melicit(arc)
            }
            // ============================================================
            /// weak
            pub fn weak(&self) -> WeakMelicit {
                WeakMelicit(Arc::downgrade(&self.0))
            }
            // ============================================================
            /// lock
            pub fn lock(
                &self,
            ) -> LockResult<MutexGuard<'_, Box<dyn MelicitBase>>>
            {
                self.0.lock()
            }
            // ============================================================
            /// try_lock
            pub fn try_lock(
                &self,
            ) -> TryLockResult<MutexGuard<'_, Box<dyn MelicitBase>>>
            where
                dyn MelicitBase: Debug + MelicitFromSelf,
            {
                self.0.try_lock()
            }
            // ============================================================
            /// with
            pub fn with<T, E>(
                &self,
                f: impl FnOnce(&dyn MelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.lock().map_or_else(
                    |_| Err(E::from(ElicitError::Poisoned)),
                    |x| f((*x).as_ref())
                )
            }
            // ============================================================
            /// try_with
            pub fn try_with<T, E>(
                &self,
                f: impl FnOnce(&dyn MelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.try_lock().map_or_else(
                    |x| match x {
                        TryLockError::Poisoned(_) => {
                            Err(E::from(ElicitError::Poisoned))
                        }
                        TryLockError::WouldBlock => {
                            Err(E::from(ElicitError::WouldBlock))
                        }
                    },
                    |x| f((*x).as_ref())
                )
            }
            // ============================================================
            /// with_mut
            pub fn with_mut<T, E>(
                &self,
                f: impl FnOnce(&mut dyn MelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.lock().map_or_else(
                    |_| Err(E::from(ElicitError::Poisoned)),
                    |mut x| f((*x).as_mut())
                )
            }
            // ============================================================
            /// try_with_mut
            pub fn try_with_mut<T, E>(
                &self,
                f: impl FnOnce(&mut dyn MelicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<ElicitError>,
            {
                self.try_lock().map_or_else(
                    |x| match x {
                        TryLockError::Poisoned(_) => {
                            Err(E::from(ElicitError::Poisoned))
                        }
                        TryLockError::WouldBlock => {
                            Err(E::from(ElicitError::WouldBlock))
                        }
                    },
                    |mut x| f((*x).as_mut())
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
