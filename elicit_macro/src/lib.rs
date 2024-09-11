// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2024 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/10
//  @date 2024/09/11

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.81.0 (eeb90cda1 2024-09-04)
#![forbid(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    deprecated_safe,
    edition_2024_expr_fragment_specifier,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents_2018,
    keyword_idents_2024,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_unsafe_on_extern,
    non_ascii_idents,
    non_local_definitions,
    redundant_lifetimes,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unnameable_types,
    unreachable_pub,
    unsafe_attr_outside_unsafe,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    ambiguous_glob_imports,
    ambiguous_glob_reexports,
    ambiguous_wide_pointer_comparisons,
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    async_fn_in_trait,
    bad_asm_style,
    bare_trait_objects,
    boxed_slice_into_iter,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    clashing_extern_declarations,
    coherence_leak_check,
    confusable_idents,
    const_evaluatable_unchecked,
    const_eval_mutable_ptr_in_final_value,
    const_item_mutation,
    dead_code,
    dependency_on_unit_never_type_fallback,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    dropping_copy_types,
    dropping_references,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    forbidden_lint_groups,
    forgetting_copy_types,
    forgetting_references,
    for_loops_over_fallibles,
    function_item_references,
    hidden_glob_reexports,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    inline_no_sanitize,
    internal_features,
    invalid_from_utf8,
    invalid_macro_export_arguments,
    invalid_nan_comparisons,
    invalid_value,
    irrefutable_let_patterns,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    map_unit_fn,
    mixed_script_confusables,
    named_arguments_used_positionally,
    never_type_fallback_flowing_into_unsafe,
    non_camel_case_types,
    non_contiguous_range_endpoints,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    noop_method_call,
    no_mangle_generic_items,
    opaque_hidden_inferred_bound,
    out_of_scope_macro_calls,
    overlapping_range_endpoints,
    path_statements,
    private_bounds,
    private_interfaces,
    ptr_cast_add_auto_to_object,
    redundant_semicolons,
    refining_impl_trait_internal,
    refining_impl_trait_reachable,
    renamed_and_removed_lints,
    repr_transparent_external_private_fields,
    self_constructor_from_outer_item,
    semicolon_in_expressions_from_macros,
    special_module_name,
    stable_features,
    static_mut_refs,
    suspicious_double_ref_op,
    temporary_cstring_as_ptr,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    uncovered_param_in_projection,
    undefined_naked_function_abi,
    unexpected_cfgs,
    unfulfilled_lint_expectations,
    ungated_async_fn_track_caller,
    uninhabited_static,
    unknown_lints,
    unknown_or_malformed_diagnostic_attributes,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_associated_type_bounds,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_labels,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    useless_ptr_null_checks,
    wasm_c_abi,
    while_true,
    writes_through_immutable_pointer,
    ambiguous_associated_items,
    arithmetic_overflow,
    binary_asm_labels,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    conflicting_repr_hints,
    deprecated_cfg_attr_crate_type_name,
    elided_lifetimes_in_associated_constant,
    enum_intrinsics_non_enums,
    ill_formed_attribute_input,
    incomplete_include,
    ineffective_unstable_trait_impl,
    invalid_atomic_ordering,
    invalid_doc_attributes,
    invalid_from_utf8_unchecked,
    invalid_reference_casting,
    invalid_type_param_default,
    let_underscore_lock,
    long_running_const_eval,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    named_asm_labels,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    patterns_in_fns_without_body,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    soft_unstable,
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    unconditional_panic,
    undropped_manually_drops,
    unknown_crate_types,
    useless_deprecated
)]
#![deny(deprecated)]
// ////////////////////////////////////////////////////////////////////////////
// mod  =======================================================================
mod find_field_attr;
// ============================================================================
///
/// mod include
///
/// proc_macro::TokenStream is not included in crate::include to suppress
/// "procedural macro API is used outside of a procedural macro".
///
pub(crate) mod include {
    // common  ----------------------------------------------------------------
    pub(crate) use proc_macro2::{Span, TokenStream as TokenStream2};
    pub(crate) use quote::{quote, ToTokens};
    pub(crate) use syn::{parse_macro_input, Error};

    pub(crate) type Result<T> = std::result::Result<T, Error>;

    #[inline]
    pub(crate) fn into_tokens(
        res: Result<TokenStream2>,
    ) -> proc_macro::TokenStream {
        res.unwrap_or_else(Error::into_compile_error).into()
    }

    // for elicit_macro  ------------------------------------------------------
    pub(crate) use syn::{DeriveInput, Ident, ItemTrait};
}
// ============================================================================
mod aelicit_derive;
mod elicit_define;
mod elicit_derive;
mod melicit_derive;
// ============================================================================
#[cfg(feature = "parking_lot")]
mod feature_parking_lot;
#[cfg(feature = "parking_lot")]
use feature_parking_lot::{aelicit_define, melicit_define};
// ----------------------------------------------------------------------------
#[cfg(not(any(feature = "parking_lot",)))]
mod feature_default;
#[cfg(not(any(feature = "parking_lot",)))]
use feature_default::{aelicit_define, melicit_define};
// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use crate::include::*;
use proc_macro::TokenStream;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// ```compile_fail
/// #[elicit_define(MODULE)]
/// trait Base {..}
/// ```
///
#[proc_macro_attribute]
pub fn elicit_define(attr: TokenStream, item: TokenStream) -> TokenStream {
    into_tokens(elicit_define::expand(
        parse_macro_input!(attr as Ident),
        parse_macro_input!(item as ItemTrait),
    ))
}
// ============================================================================
///
/// ```compile_fail
/// #[derive(Debug, Elicit)]
/// #[elicit_mod_author(MODULE)]        // required
/// #[elicit_from_self_field(_fsf)]     // option here
/// struct Derived {
///   #[elicit_from_self_field]         // or here
///   _fsf: ElicitFromSelfField,
/// }
/// ```
///
#[proc_macro_derive(
    Elicit,
    attributes(elicit_mod_author, elicit_from_self_field)
)]
pub fn on_elicit_derive(ts: TokenStream) -> TokenStream {
    into_tokens(elicit_derive::expand(parse_macro_input!(ts as DeriveInput)))
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// ```compile_fail
/// #[aelicit_define(MODULE)]
/// trait Base {..}
/// ```
///
#[proc_macro_attribute]
pub fn aelicit_define(attr: TokenStream, item: TokenStream) -> TokenStream {
    into_tokens(aelicit_define::expand(
        parse_macro_input!(attr as Ident),
        parse_macro_input!(item as ItemTrait),
    ))
}
// ============================================================================
///
/// ```compile_fail
/// #[derive(Debug, Aelicit)]
/// #[aelicit_mod_author(MODULE)]        // required
/// #[aelicit_from_self_field(_fsf)]     // option here
/// struct Derived {
///   #[aelicit_from_self_field]         // or here
///   _fsf: AelicitFromSelfField,
/// }
/// ```
///
#[proc_macro_derive(
    Aelicit,
    attributes(aelicit_mod_author, aelicit_from_self_field)
)]
pub fn on_aelicit_derive(ts: TokenStream) -> TokenStream {
    into_tokens(aelicit_derive::expand(parse_macro_input!(
        ts as DeriveInput
    )))
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
///
/// ```compile_fail
/// #[melicit_define(MODULE)]
/// trait Base {..}
/// ```
///
#[proc_macro_attribute]
pub fn melicit_define(attr: TokenStream, item: TokenStream) -> TokenStream {
    into_tokens(melicit_define::expand(
        parse_macro_input!(attr as Ident),
        parse_macro_input!(item as ItemTrait),
    ))
}
// ============================================================================
///
/// ```compile_fail
/// #[derive(Debug, Melicit)]
/// #[melicit_mod_author(MODULE)]        // required
/// #[melicit_from_self_field(_fsf)]     // option here
/// struct Derived {
///   #[melicit_from_self_field]         // or here
///   _fsf: MelicitFromSelfField,
/// }
/// ```
///
#[proc_macro_derive(
    Melicit,
    attributes(melicit_mod_author, melicit_from_self_field)
)]
pub fn on_melicit_derive(ts: TokenStream) -> TokenStream {
    into_tokens(melicit_derive::expand(parse_macro_input!(
        ts as DeriveInput
    )))
}
