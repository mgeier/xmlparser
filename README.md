## xmlparser
[![Build Status](https://travis-ci.org/RazrFalcon/xmlparser.svg?branch=master)](https://travis-ci.org/RazrFalcon/xmlparser)
[![Crates.io](https://img.shields.io/crates/v/xmlparser.svg)](https://crates.io/crates/xmlparser)
[![Documentation](https://docs.rs/xmlparser/badge.svg)](https://docs.rs/xmlparser)
[![Rust 1.18+](https://img.shields.io/badge/rust-1.18+-orange.svg)](https://www.rust-lang.org)


*xmlparser* is a low-level, pull-based, zero-allocation
[XML 1.0](https://www.w3.org/TR/xml/) parser.

### Example

```rust
for token in xmlparser::Tokenizer::from("<tagname name='value'/>") {
    println!("{:?}", token);
}
```

### Why a new library

This library is basically a low-level XML tokenizer that preserves a position of the tokens
and does not intend to be used directly.
If you are looking for a more high-level solution - checkout
[roxmltree](https://github.com/RazrFalcon/roxmltree).

### Benefits

- All tokens contain `StrSpan` objects which contain a position of the data in the original document.
- Good error processing. All error types contain position (line:column) where it occurred.
- No heap allocations.
- No dependencies.
- Tiny. ~1500 LOC and ~40KiB in the release build according to the `cargo-bloat`.

### Limitations

- Currently, only ENTITY objects are parsed from the DOCTYPE. Other ignored.
- No tree structure validation. So an XML like `<root><child></root></child>`
  will be parsed without errors. You should check for this manually.
  On the other hand `<a/><a/>` will lead to an error.
- Duplicated attributes is not an error. So an XML like `<item a="v1" a="v2"/>`
  will be parsed without errors. You should check for this manually.
- UTF-8 only.

### Safety

- The library must not panic. Any panic considered as a critical bug
  and should be reported.
- The library forbids the unsafe code.

### License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
