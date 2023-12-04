<!--
SPDX-FileCopyrightText: 2022 - 2023 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# codify-rs

[![License: AGPL-3.0-or-later](
    https://img.shields.io/badge/License-AGPL%203.0+-blue.svg)](
    LICENSE.txt)
[![REUSE status](
    https://api.reuse.software/badge/github.com/hoijui/codify-rs)](
    https://api.reuse.software/info/github.com/hoijui/codify-rs)

Helps in automatic code generation for initializing structs and enums.

Commonly used like this:

1. in your `build.rs`, load some data from resource files into struct, using serde
2. in your `build.rs`, call the `Codify::init_code()` function on that data,
    creating init code, and write that code into a `*.rs` file
3. load/use that `*.rs` file at compile-time

For an example usage,
see the [build.rs](https://github.com/hoijui/osh-dir-std-rs/blob/master/build.rs)
file of project [osh-dir-std-rs](https://github.com/hoijui/osh-dir-std-rs/).
