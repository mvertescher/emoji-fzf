# `emoji-fzf`

[![Build Status](https://travis-ci.com/mvertescher/emoji-fzf.svg?branch=master)](https://travis-ci.com/mvertescher/emoji-fzf)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> 🦀 An emoji fuzzy finder written in Rust.

Inspired by an excellent tool of the same name: [emoji-fzf](https://github.com/noahp/emoji-fzf)!

## Usage

```plaintext
emoji-fzf 0.1.0
Matt Vertescher <mvertescher@gmail.com>
An emoji fuzzy finder!

USAGE:
    emoji-fzf [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get     Get unicode emoji given a name
    help    Prints this message or the help of the given subcommand(s)
```

Or the alias form integrated with fzf (include in your shell rc), and piping
the result to xclip:

```bash
alias emoj="emoji-fzf preview | fzf --preview 'emoji-fzf get {1}' | cut -d \" \" -f 1 | emoji-fzf get | xclip"
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
