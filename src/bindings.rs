// SPDX-License-Identifier: GPL-2
//
// Copyright (C) 2021  William Findlay
//
// Jan. 19, 2021  William Findlay  Created this.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

/// Raw bindings generated by bindgen
mod gen {
    include!("libbpfcontain/bindings.rs");
}

// TODO: Export your bindings here
