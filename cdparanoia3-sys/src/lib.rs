// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: LGPL-2.1-or-later

//! Low-level bindings for cdparanoia-3.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// cdparanoia-3 doesn't export a `cdda_free_messages()`,
// so we re-export libc which provides `free()`
#[cfg(feature = "libc")]
pub use libc;
