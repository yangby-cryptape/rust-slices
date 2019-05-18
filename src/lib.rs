// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A series of macros that used to construct static unsigned integer slices from string literals.
//!
//! # Usage
//!
//! The input is a hexadecimal string literal with `0x` prefix.
//! The size of input should be an even number.
//!
//! And you can use any number of `_` in the string literal to separate it for more readable.
//!
//! ## Examples
//!
//! ```rust
//! use slices::u8_slice;
//!
//! const VAL: &[u8] = u8_slice!("0x_1234_5678_9abc_def0");
//! const NULL: &[u8] = u8_slice!("0x");
//!
//! fn main () -> ::std::io::Result<()> {
//!     let val = &[0x12u8, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
//!     assert_eq!(VAL, val);
//!     assert_eq!(NULL, &[]);
//!     Ok(())
//! }
//! ```

use proc_macro_hack::proc_macro_hack;

/// A macro used to construct static u8 slices from string literals.
#[proc_macro_hack]
pub use hack::u8_slice;
