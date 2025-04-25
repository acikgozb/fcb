# fcb

[![CI](https://github.com/acikgozb/fcb/actions/workflows/ci.yml/badge.svg)](https://github.com/acikgozb/fcb/actions/workflows/ci.yml) ![version](https://img.shields.io/badge/version-0.1.0-red) ![release](https://img.shields.io/badge/release-stable-89e051)

Generates fenced code blocks.

## Table of Contents

<!--toc:start-->
  - [Installation](#installation)
    - [Build From Source](#build-from-source)
    - [Prebuilt Binaries](#prebuilt-binaries)
  - [Usage](#usage)
    - [Intended Usage](#intended-usage)
  - [TODO](#todo)
  - [LICENSE](#license)
<!--toc:end-->

## <a id='installation'></a> Installation

There are 2 ways to install `fcb`.

### <a id='build-from-source'></a> Build From Source

If you have `cargo` installed on your host, you can use it to build `fcb` from source.
For Windows, this is the only way to install `fcb`.

```bash
# Clone the repository.
git clone git@github.com:acikgozb/fcb.git ./fcb

# Install via `cargo`
cd ./fcb
cargo build --release --locked 

# Put the binary under $PATH.
# In here, it is assumed that ~/.local/bin is on $PATH.
cp ./target/release/fcb ~/.local/bin/fcb

# Validate the $PATH lookup before using fcb.
which fcb
```

### <a id='prebuilt-binaries'></a> Prebuilt Binaries

You can also install `fcb` by downloading prebuilt binaries from the [releases page](https://github.com/acikgozb/fcb/releases).

Extract `fcb` from its archive, and then put it under `$PATH` like above.

## <a id='usage'></a> Usage

Here are the basic, standalone usages of the tool:

```bash
# Check help before starting out.
fcb -h

# Empty block.
fcb

# Empty block with language.
fcb -l rust

# Block with language & content.
fcb -l rust 'println!();'

# Block with piped content.
echo -n 'println!(\"Hello world!\");' | fcb
curl -sSL https://jsonplaceholder.typicode.com/todos/1 | fcb -l json | clip

# Block with redirected content.
fcb -l rust < content.rs
fcb -l rust <<< 'println!("Hello world!");'
fcb -l rust < <(echo -n 'println!("Hello world!");')

# Block with redirected content
# that is saved to a file.
fcb -l rust < content.rs > block.txt
```

### <a id='intended-usage'></a> Intended Usage

Typing out code for a command is not very intuitive:

- The code may be several lines long.
- Due to the nature of shell expansion, special characters
inside the code may need to be escaped.

Therefore, the main way of using `fcb` is actually through editors like Helix and (Neo)Vim since they allow users to use command line tools for code selections.

As an example, you can define a keymap in Helix that calls `fcb` for the selection, and wrap it with fenced code blocks.

Helix's `config.toml`:

```toml
# Save a macro that prefills the
# command prompt with fcb.

[keys.normal."'"]
b = "@:pipe fcb -l"

[keys.select."'"]
b = "@:pipe fcb -l"
```

Editor buffer:

~~~rust
// Assume that the code below
// needs to be wrapped with
// a code block.
println!("Hello world!");

// Flow:
// 1 - Select the code.
// 2 - Type 'b in normal or select mode.
// 3 - Set LANG if desired (e.g rust).
// Result:
// ```rust
// println!("Hello world!");
// ```
~~~

## <a id='todo'></a> TODO

- Demos for standalone + editor usage.
- `man fcb`
- Add tilde as an alternative delimiter.
- Publish crate docs.

## <a id='license'></a> LICENSE

This work is dual-licensed under Apache 2.0 and GPL 2.0 (or any later version).
You can choose between one of them if you use this work.

`SPDX-License-Identifier: Apache-2.0 OR GPL-2.0-or-later`
