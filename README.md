{%- comment %} 
# longan-nano-template

Template for Rust projects on the Longan Nano development board.

## New projects

Install cargo-generate:

```shell
cargo install cargo-generate
```

Generate a new project:

```shell
cargo generate --git https://github.com/Yoshanuikabundi/rust-template.git -b main -n "<name of new project>"
```

Leave blank any prompts where you have privacy concerns. From cargo-generate version 0.9.0, the following should work:

```shell
cargo generate --git https://github.com/Yoshanuikabundi/rust-template.git -b main -n "<name of new project>" -d licensor="<your name>" -d github_username="<your username>"
```

{% endcomment -%}

{{"#"}} {{project-name}}

{% if github_username != "" %}
[![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/{{github_username}}/{{project-name}}?label=tag&logo=github&sort=semver)](https://github.com/{{github_username}}/{{project-name}})
[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})
[![CI](https://github.com/{{github_username}}/{{project-name}}/workflows/Continuous%20Integration/badge.svg)](https://github.com/{{github_username}}/{{project-name}}/actions)
[![Coverage Status](https://coveralls.io/repos/github/{{github_username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{github_username}}/{{project-name}}?branch=main)
[![MIT License](https://img.shields.io/github/license/yoshanuikabundi/{{project-name}})](https://github.com/{{github_username}}/{{project-name}}/blob/main/LICENSE-MIT)

{% endif -%}

## Building projects

The easiest way to build {{project-name}} requires a Sipeed JTAG adapter. For methods that do not require this hardware, see the [Longan Nano board crate readme](https://github.com/riscv-rust/longan-nano).

### External Dependencies

- Rust (naturally) 1.54 or newer with the RISC-V target. Works on Stable!
- OpenOCD for RISC-V ([on GitHub](https://github.com/riscv/riscv-openocd))
- The RISC-V toolchain, or at least GDB ([e.g. from SiFive](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz))

These are all pretty easy to install on Arch:

```shell
pacman -Syu rustup paru
paru -S riscv-openocd-git  riscv-sifive-elf-gdb 
rustup target add riscv32imac-unknown-none-elf
rustup update stable
```

## License (MIT)

Copyright 2019-2020 [RISC-V team][team]
Copyright {{ "now" | date: "%Y" }} {{licensor}}

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC]. The maintainer of this crate promises to intervene to uphold
that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
