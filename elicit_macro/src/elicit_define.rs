// -*- mode:rust; coding:utf-8-unix; -*-

//! elicit_define.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/11
//  @date 2024/04/14

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
        #[allow(box_pointers, dead_code)]
        mod #mod_ident {
            mod _inner { #inner }

            mod _common {
                pub use super::_inner::{
                    Elicit, ElicitBase, ElicitFromSelf,
                };
            }

            pub mod author {
                pub use super::_common::*;
                pub use super::_inner::{
                    WeakAssign,
                    ElicitFromSelfField
                };
            }

            pub mod user {
                pub use super::_common::*;
                pub use super::_inner::WeakElicit;
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
        use elicit::Error;
        use std::{
            cell::RefCell,
            convert::From,
            fmt::Debug,
            rc::{Rc, Weak},
            result::Result as StdResult,
        };
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        pub trait ElicitBase:
            'static + Debug + #orig + ElicitFromSelf + WeakAssign
        {
        }
        impl<T: 'static + Debug + #orig + ElicitFromSelf + WeakAssign>
            ElicitBase for T
        {
        }
        // ================================================================
        /// struct Elicit
        #[derive(Debug, Clone)]
        pub struct Elicit(Rc<RefCell<Box<dyn ElicitBase>>>);
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct WeakElicit
        #[derive(Debug, Clone)]
        pub struct WeakElicit(Weak<RefCell<Box<dyn ElicitBase>>>);
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
        /// trait ElicitFromSelf
        pub trait ElicitFromSelf {
            /// elicit_from_self
            fn elicit_from_self(&self) -> Option<Elicit>;
        }
        // ================================================================
        /// trait WeakAssign
        pub trait WeakAssign {
            /// _weak_assign
            fn _weak_assign(
                &mut self,
                weak: Weak<RefCell<Box<dyn ElicitBase>>>,
            );
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        /// struct ElicitFromSelfField
        #[derive(Debug, Clone, Default)]
        pub struct ElicitFromSelfField {
            /// _weak
            _weak: Option<Weak<RefCell<Box<dyn ElicitBase>>>>,
        }
        // ================================================================
        impl ElicitFromSelf for ElicitFromSelfField {
            /// elicit_from_self
            fn elicit_from_self(&self) -> Option<Elicit> {
                match self._weak {
                    Some(ref x) => x.upgrade().map(Elicit),
                    None => None,
                }
            }
        }
        // ================================================================
        impl WeakAssign for ElicitFromSelfField {
            /// _weak_assign
            fn _weak_assign(
                &mut self,
                weak: Weak<RefCell<Box<dyn ElicitBase>>>,
            ) {
                self._weak = Some(weak)
            }
        }
        // ////////////////////////////////////////////////////////////////
        // ================================================================
        impl Elicit {
            // ============================================================
            /// new
            #[allow(trivial_casts)]
            pub fn new<T>(val: T) -> Self
            where
                T: ElicitBase,
            {
                let rc = Rc::new(RefCell::new(
                    Box::new(val) as Box<dyn ElicitBase>
                ));
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
                f: impl FnOnce(&dyn ElicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<Error>,
            {
                f(&(*(*(self.0.as_ref().borrow()))))
            }
            // ============================================================
            /// with_mut
            pub fn with_mut<T, E>(
                &self,
                f: impl FnOnce(&mut dyn ElicitBase) -> StdResult<T, E>,
            ) -> StdResult<T, E>
            where
                E: From<Error>,
            {
                f(&mut (*(*(self.0.as_ref().borrow_mut()))))
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
