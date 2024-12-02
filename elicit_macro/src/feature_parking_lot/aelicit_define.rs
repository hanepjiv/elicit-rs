// -*- mode:rust; coding:utf-8-unix; -*-

//! `aelicit_define.rs`

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/14
//  @date 2024/11/30

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::{
    Ident, ItemTrait, Result, ToTokens, TokenStream2, quote,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// fn expand
pub fn expand(mod_ident: &Ident, item: ItemTrait) -> Result<TokenStream2> {
    let define = quote_define(mod_ident, &item)?;
    let mut ret = item.into_token_stream();
    ret.extend(define);
    Ok(ret)
}
// ----------------------------------------------------------------------------
/// fn `quote_define`
#[allow(clippy::unnecessary_wraps)]
fn quote_define(mod_ident: &Ident, item: &ItemTrait) -> Result<TokenStream2> {
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
        WeakAelicitInner,
        AelicitFromSelfField
    };
    }

    /// mod user
    pub mod user {
    pub use super::_common::*;
    pub use super::_inner::{
        WeakAelicit,
        ReadGuard, WriteGuard
    };
    }
    }
    })
}
// ----------------------------------------------------------------------------
/// fn `quote_inner`
#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
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
    sync::{ OnceLock, Arc, Weak, },
    };
    use elicit::RwLock;
    // --------------------------------------------------------------------
    pub use elicit::{ Result as ElicitResult, Error as ElicitError };
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// trait AelicitBase
    pub trait AelicitBase: 'static + Unpin + Debug
    + #orig + AelicitFromSelf + WeakAssign
    {
    // ================================================================
    /// usizeptr
    fn usizeptr(&self) -> usize;
    }
    // ====================================================================
    impl<T: 'static + Unpin + Debug + #orig + AelicitFromSelf + WeakAssign>
    AelicitBase for T
    {
    // ================================================================
    #[allow(trivial_casts)]
    fn usizeptr(&self) -> usize {
    self as *const _ as usize
    }
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    type RwLockInner = Pin<Box<dyn AelicitBase>>;
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// type ReadGuard
    pub type ReadGuard<'a> = elicit::RwLockReadGuard<'a, RwLockInner>;
    // --------------------------------------------------------------------
    /// type WriteGuard
    pub type WriteGuard<'a> = elicit::RwLockWriteGuard<'a, RwLockInner>;
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    type AelicitInner = Arc<RwLock<RwLockInner>>;
    // --------------------------------------------------------------------
    /// struct Aelicit
    #[derive(Debug, Clone)]
    pub struct Aelicit(AelicitInner);
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// type WeakAelicitInner
    pub type WeakAelicitInner = Weak<RwLock<RwLockInner>>;
    // --------------------------------------------------------------------
    /// struct WeakAelicit
    #[derive(Debug, Clone)]
    pub struct WeakAelicit(WeakAelicitInner);
    // ====================================================================
    impl WeakAelicit {
    // ================================================================
    /// fn upgrade
    pub fn upgrade(&self) -> Option<Aelicit> {
    self.0.upgrade().map(Aelicit)
    }
    }
    // ====================================================================
    impl From<Aelicit> for WeakAelicit {
    fn from(x: Aelicit) -> WeakAelicit {
    x.weak()
    }
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// trait AelicitFromSelf
    pub trait AelicitFromSelf {
    /// aelicit_from_self
    fn aelicit_from_self(&self) -> Option<Aelicit>;
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// trait WeakAssign
    pub trait WeakAssign {
    /// _weak_assign
    fn _weak_assign(
    &mut self,
    weak: WeakAelicitInner,
    ) -> ElicitResult<()>;
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    /// struct AelicitFromSelfField
    #[derive(Debug, Clone, Default)]
    pub struct AelicitFromSelfField {
    /// _weak
    _weak: OnceLock<WeakAelicitInner>,
    }
    // ====================================================================
    impl AelicitFromSelf for AelicitFromSelfField {
    fn aelicit_from_self(&self) -> Option<Aelicit> {
    self._weak.get()?.upgrade().map(Aelicit)
    }
    }
    // ====================================================================
    impl WeakAssign for AelicitFromSelfField {
    fn _weak_assign(
    &mut self,
    weak: WeakAelicitInner,
    ) -> ElicitResult<()> {
    self._weak.set(weak).map_err(
        |_| ElicitError::WeakAlreadyExists)
    }
    }
    // ////////////////////////////////////////////////////////////////////
    // ====================================================================
    impl Aelicit {
    // ================================================================
    /// new
    #[allow(trivial_casts)]
    pub fn new<T>(val: T) -> ElicitResult<Self>
    where
    T: AelicitBase,
    {
    let r = Arc::new(RwLock::new(Box::pin(val) as RwLockInner));
    r.write().as_mut()._weak_assign(Arc::<_>::downgrade(&r))?;
    Ok(Aelicit(r))
    }
    // ================================================================
    /// weak
    pub fn weak(&self) -> WeakAelicit {
    WeakAelicit(Arc::downgrade(&self.0))
    }
    // ================================================================
    /// usizeptr
    pub fn usizeptr(&self) -> usize {
    self.0.read().as_ref().usizeptr()
    }
    // ================================================================
    /// read
    pub fn read(&self) -> ReadGuard<'_> {
    self.0.read()
    }
    // ================================================================
    /// try_read
    pub fn try_read(&self) -> Option<ReadGuard<'_>> {
    self.0.try_read()
    }
    // ================================================================
    /// write
    pub fn write(&self) -> WriteGuard<'_> {
    self.0.write()
    }
    // ================================================================
    /// try_write
    pub fn try_write(&self) -> Option<WriteGuard<'_>> {
    self.0.try_write()
    }
    // ================================================================
    /// with
    pub fn with<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&dyn AelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    {
    f(self.read().as_ref().get_ref())
    }
    // ================================================================
    /// try_with
    pub fn try_with<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&dyn AelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    E: From<ElicitError>,
    {
    if let Some(x) = self.try_read() {
        f(x.as_ref().get_ref())
    } else {
        Err(ElicitError::WouldBlock.into())
    }
    }
    // ================================================================
    /// with_mut
    pub fn with_mut<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&mut dyn AelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    {
    f(self.write().as_mut().get_mut())
    }
    // ================================================================
    /// try_with_mut
    pub fn try_with_mut<'s, 'a, T, E>(
    &'s self,
    f: impl FnOnce(&mut dyn AelicitBase) -> StdResult<T, E>,
    ) -> StdResult<T, E>
    where
    's: 'a,
    E: From<ElicitError>,
    {
    if let Some(mut x) = self.try_write() {
        f(x.as_mut().get_mut())
    } else {
        Err(ElicitError::WouldBlock.into())
    }
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
        assert!(
            expand(
                &parse2(quote!(mod_ident)).expect("parse attr"),
                parse2(quote!(
                    trait Orig {}
                ))
                .expect("parse item")
            )
            .is_ok()
        );
    }
}
