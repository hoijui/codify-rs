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

The practical use case of this,
is to store resources/data in the binary in the most efficient form,
so it will be ready at runtime,
without having to include the data/files
in their original, serialized, potentially innefficient form
into the binary.
This also means, the data does not have to be parsed
at runtime/application-startup.

## Usage

0. Add this library to your normal and build dependencies in cargo:
    ```
    [dependencies]
    codify = { version = "0.3", package = "codify_hoijui" }

    [build-dependencies]
    codify = { version = "0.3", package = "codify_hoijui" }
    ```
1. in your `build.rs`, load some data from resource files
    into structs/enums, using serde
2. in your `build.rs`, call `Codify::init_code()` on that data,
    creating init code.
3. write that code into a `$OUT_DIR/my_data_x_gen.rs` file
4. Create a proxy source file under `src/` -
    for example `src/my_data_x_gen.rs` -
    which does nothing more then include the generated file:
    `include!(concat!(env!("OUT_DIR"), "/my_data_x_gen.rs"));`
5. load/use `$OUT_DIR/my_data_x.rs` file at compile-time

## Example

For an example usage,
see the [build.rs](
https://github.com/hoijui/osh-dir-std-rs/blob/master/build.rs)
file of [osh-dir-std-rs](
https://github.com/hoijui/osh-dir-std-rs/) project.
