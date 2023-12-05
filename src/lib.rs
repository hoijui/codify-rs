// SPDX-FileCopyrightText: 2022 - 2023 Robin Vobruba <hoijui.quaero@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

pub trait Codify {
    fn init_code(&self) -> Cow<'static, str>;
}

impl<T: Codify> Codify for Option<T> {
    fn init_code(&self) -> Cow<'static, str> {
        self.as_ref().map_or(Cow::Borrowed("None"), |val| {
            Cow::Owned(format!("Some({})", val.init_code()))
        })
    }
}

impl<'a, T> Codify for Cow<'a, T>
where
    T: Clone,
    for<'b> &'b T: Codify,
{
    fn init_code(&self) -> Cow<'static, str> {
        self.as_ref().init_code()
    }
}

impl<T> Codify for Box<T>
where
    for<'b> &'b T: Codify,
{
    fn init_code(&self) -> Cow<'static, str> {
        self.as_ref().init_code()
    }
}

// impl<T: AsRef<str>> Codify for T {
//     fn init_code(&self) -> Cow<'static, str> {
//         Cow::Owned(format!(r##""{}""##, self.as_ref()))
//     }
// }

impl Codify for String {
    fn init_code(&self) -> Cow<'static, str> {
        // NOTE: This codifies the String into a &'static str!
        Cow::Owned(format!(r##"String::from("{self}")"##))
    }
}

impl Codify for &str {
    fn init_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!(r##""{self}""##))
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

impl<E, S: ::std::hash::BuildHasher> Codify for HashSet<E, S>
where
    E: Codify,
{
    fn init_code(&self) -> Cow<'static, str> {
        let mut parts = vec!["{{\nlet mut set = HashSet::new();\n".to_string()];
        for entry in self {
            parts.push(format!(
                r##"set.insert({});
"##,
                entry.init_code(),
            ));
        }
        parts.push("set\n}}".to_string());

        Cow::Owned(parts.concat())
    }
}

// This tests rust code in the README with doc-tests.
// Though, It will not appear in the generated documentaton.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
