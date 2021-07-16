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

extern crate proc_macro;

use quote::quote;
use syn::parse_macro_input;

struct Bytes(Vec<u8>);

impl ::std::fmt::Debug for Bytes {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let data = &self.0;
        write!(f, "&[")?;
        if !data.is_empty() {
            write!(f, "{:#04x}u8", data[0])?;
            for unit in data.iter().skip(1) {
                write!(f, ", {:#04x}", unit)?;
            }
        }
        write!(f, "]")
    }
}

/// A macro used to construct static u8 slices from string literals.
#[proc_macro]
pub fn u8_slice(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);
    let expanded = {
        let input = input.value().replace("_", "");
        if input.len() < 2 || &input[..2] != "0x" || input.len() % 2 != 0 {
            panic!("Input has to be a hexadecimal string with 0x-prefix.");
        };
        let input_str = &input[2..];
        let buffer_len = input_str.len() / 2;
        let buffer = if buffer_len != 0 {
            let mut buffer = vec![0u8; buffer_len];
            faster_hex::hex_decode(input_str.as_bytes(), &mut buffer).unwrap_or_else(|err| {
                panic!("Failed to parse the input hexadecimal string: {}", err);
            });
            buffer
        } else {
            vec![]
        };
        let bytes = Bytes(buffer);
        let eval_str = format!("{:?}", bytes);
        let eval_ts: proc_macro2::TokenStream = eval_str.parse().unwrap_or_else(|_| {
            panic!(
                "Failed to parse the string \"{}\" to TokenStream.",
                eval_str
            );
        });
        quote!(#eval_ts)
    };
    expanded.into()
}
