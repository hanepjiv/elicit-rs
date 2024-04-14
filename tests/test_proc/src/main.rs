// -*- mode:rust; coding:utf-8-unix; -*-

//! do_elicit.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2024/04/13
//  @date 2024/04/14

// ////////////////////////////////////////////////////////////////////////////
mod do_aelicit;
mod do_elicit;
mod do_melicit;
// ============================================================================
fn main() {
    do_elicit::fire();
    do_aelicit::fire();
    do_melicit::fire();
}
