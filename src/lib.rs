// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![warn(rust_2021_compatibility)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::wildcard_enum_match_arm)]
#![warn(clippy::string_slice)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::try_err)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::use_debug)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::trivial_regex)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::fn_params_excessive_bools)]

use std::{borrow::Cow, collections::HashMap};

pub trait Codify {
    fn init_code(&self) -> Cow<'static, str>;
}

// impl<T: AsRef<str>> Codify for T {
//     fn init_code(&self) -> Cow<'static, str> {
//         Cow::Owned(format!(r##""{}""##, self.as_ref()))
//     }
// }

impl Codify for String {
    fn init_code(&self) -> Cow<'static, str> {
        // NOTE: This codifies the String into a &'static str!
        Cow::Owned(format!(r##""{}""##, self))
    }
}

impl Codify for &str {
    fn init_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!(r##""{}""##, self))
    }
}

impl<T> Codify for Vec<T>
where
    T: Codify,
{
    fn init_code(&self) -> Cow<'static, str> {
        let mut parts = vec!["vec![\n".to_string()];
        for entry in self {
            parts.push(format!("{},\n", entry.init_code()));
        }
        parts.push("]".to_string());

        Cow::Owned(parts.concat())
    }
}

impl<K, V, S: ::std::hash::BuildHasher> Codify for HashMap<K, V, S>
where
    K: Codify,
    V: Codify,
{
    fn init_code(&self) -> Cow<'static, str> {
        let mut parts = vec!["{{\nlet mut map = HashMap::new();\n".to_string()];
        for (key, val) in self {
            parts.push(format!(
                r##"map.insert({}, {});
"##,
                key.init_code(),
                val.init_code()
            ));
        }
        parts.push("map\n}}".to_string());

        Cow::Owned(parts.concat())
    }
}
