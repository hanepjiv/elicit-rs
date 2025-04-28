// -*- mode:rust; coding:utf-8-unix; -*-

//! `melicit_define.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2025/04/28

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::{
    Ident, ItemTrait, Result, ToTokens as _, TokenStream2, quote,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// fn expand
pub(crate) fn expand(
    mod_ident: &Ident,
    item: ItemTrait,
) -> Result<TokenStream2> {
    let define = quote_define(mod_ident, &item)?;
    let mut ret = item.into_token_stream();
    ret.extend(define);
    Ok(ret)
}
// ----------------------------------------------------------------------------
/// fn `quote_define`
fn quote_define(mod_ident: &Ident, item: &ItemTrait) -> Result<TokenStream2> {
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
    WeakMelicitInner,
    MelicitFromSelfField
    };
    }

    /// mod user
    pub mod user {
    pub use super::_common::*;
    pub use super::_inner::{
    WeakMelicit,
    LockError, LockResult,
    TryLockError, TryLockResult,
    Guard,
    };
    }
    }
    })
}
// ----------------------------------------------------------------------------
/// fn `quote_inner`
#[expect(
    clippy::unnecessary_wraps,
    clippy::too_many_lines,
    reason = "checked"
)]
fn quote_inner(a_orig: &Ident) -> Result<TokenStream2> {
    let orig = quote! { super::super::#a_orig };
    Ok(quote! {
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    use std::{
    convert::From,
    fmt::Debug,
    marker::Unpin,
    pin::Pin,
    result::Result as StdResult,
    sync::{ OnceLock, Arc, Mutex, Weak },
    };
    // --------------------------------------------------------------------
    pub use std::sync::{
    LockResult, PoisonError as LockError,
    TryLockResult, TryLockError,
    };
    pub use elicit::{ Result as ElicitResult, Error as ElicitError };
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// trait MelicitBase
    pub trait MelicitBase: 'static + Unpin + Debug
    + #orig + MelicitFromSelf + WeakAssign
    {
    // ================================================================
    /// usizeptr
    fn usizeptr(&self) -> usize;
    }
    // ====================================================================
    impl<T: 'static + Unpin + Debug + #orig + MelicitFromSelf + WeakAssign>
    MelicitBase for T
    {
    // ================================================================
    #[allow(trivial_casts)]
    fn usizeptr(&self) -> usize {
    self as *const _ as usize
    }
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    type MutexInner = Pin<Box<dyn MelicitBase>>;
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// type Guard
    pub type Guard<'a> = std::sync::MutexGuard<'a, MutexInner>;
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    type MelicitInner = Arc<Mutex<MutexInner>>;
    // --------------------------------------------------------------------
    /// struct Melicit
    #[derive(Debug, Clone)]
    pub struct Melicit(MelicitInner);
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// type WeakMelicitInner
    pub type WeakMelicitInner = Weak<Mutex<MutexInner>>;
    // --------------------------------------------------------------------
    /// struct WeakMelicit
    #[derive(Debug, Clone)]
    pub struct WeakMelicit(WeakMelicitInner);
    // ====================================================================
    impl WeakMelicit {
    // ================================================================
    /// fn upgrade
    pub fn upgrade(&self) -> Option<Melicit> {
    self.0.upgrade().map(Melicit)
    }
    }
    // ====================================================================
    impl From<Melicit> for WeakMelicit {
    fn from(x: Melicit) -> WeakMelicit {
    x.weak()
    }
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// trait MelicitFromSelf
    pub trait MelicitFromSelf {
    /// melicit_from_self
    fn melicit_from_self(&self) -> Option<Melicit>;
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// trait WeakAssign
    pub trait WeakAssign {
    /// _weak_assign
    fn _weak_assign(
    &mut self,
    weak: WeakMelicitInner,
    ) -> ElicitResult<()>;
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// struct MelicitFromSelfField
    #[derive(Debug, Clone, Default)]
    pub struct MelicitFromSelfField {
    /// _weak
    _weak: OnceLock<WeakMelicitInner>,
    }
    // ====================================================================
    impl MelicitFromSelf for MelicitFromSelfField {
    fn melicit_from_self(&self) -> Option<Melicit> {
    self._weak.get()?.upgrade().map(Melicit)
    }
    }
    // ====================================================================
    impl WeakAssign for MelicitFromSelfField {
    fn _weak_assign(
    &mut self,
    weak: WeakMelicitInner,
    ) -> ElicitResult<()> {
    self._weak.set(weak).map_err(
    |_| ElicitError::WeakAlreadyExists)
    }
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    impl Melicit {
    // ================================================================
    /// new
    #[allow(trivial_casts)]
    pub fn new<T>(val: T) -> ElicitResult<Self>
    where
    T: MelicitBase,
    {
    let r = Arc::new(Mutex::new(Box::pin(val) as MutexInner));
    r.lock().expect("Melicit::new").as_mut()
    ._weak_assign(Arc::<_>::downgrade(&r))?;
    Ok(Melicit(r))
    }
    // ================================================================
    /// weak
    pub fn weak(&self) -> WeakMelicit {
    WeakMelicit(Arc::downgrade(&self.0))
    }
    // ================================================================
    /// usizeptr
    pub fn usizeptr<'s, 'a>(&'s self) ->
    StdResult<usize, LockError<Guard<'a>>>
    where
    's: 'a,
    {
    Ok(self.0.lock()?.as_ref().usizeptr())
    }
    // ================================================================
    /// lock
    pub fn lock(&self) -> LockResult<Guard<'_>>
    {
    self.0.lock()
    }
    // ================================================================
    /// try_lock
    pub fn try_lock(&self) -> TryLockResult<Guard<'_>> {
    self.0.try_lock()
    }
    // ================================================================
    /// with
    pub fn with<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&dyn MelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    E: From<LockError<Guard<'a>>>,
    {
    f(self.lock()?.as_ref().get_ref())
    }
    // ================================================================
    /// try_with
    pub fn try_with<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&dyn MelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    E: From<TryLockError<Guard<'a>>>,
    {
    f(self.try_lock()?.as_ref().get_ref())
    }
    // ================================================================
    /// with_mut
    pub fn with_mut<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&mut dyn MelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    E: From<LockError<Guard<'a>>>,
    {
    f(self.lock()?.as_mut().get_mut())
    }
    // ================================================================
    /// try_with_mut
    pub fn try_with_mut<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&mut dyn MelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    E: From<TryLockError<Guard<'a>>>,
    {
    f(self.try_lock()?.as_mut().get_mut())
    }
    }
    })
}
// ============================================================================
#[cfg(test)]
#[expect(clippy::expect_used, clippy::unwrap_used, reason = "checked")]
mod tests {
    use super::*;
    use syn::parse2;
    // ========================================================================
    #[test]
    fn case_00() {
        drop(
            expand(
                &parse2(quote!(mod_ident)).expect("parse attr"),
                parse2(quote!(
                    trait Orig {}
                ))
                .expect("parse item"),
            )
            .unwrap(),
        );
    }
}
