<h1 align="center">io-prompt-prototype</h1>
<div align="center">
  <strong>
    A prototype for io::prompt!
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/io-prompt-prototype">
    <img src="https://img.shields.io/crates/v/io-prompt-prototype.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/io-prompt-prototype">
    <img src="https://img.shields.io/crates/d/io-prompt-prototype.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/io-prompt-prototype">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/io-prompt-prototype">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/io-prompt-prototype/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/io-prompt-prototype/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

This crate builds on prior art from [this
issue](https://github.com/rust-lang/rust/pull/75435#issuecomment-695805123),
creating a convenient way to get user input from the command line.

## Installation
```sh
$ cargo add io-prompt-prototype
```

## Prior art

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/io-prompt-prototype/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/io-prompt-prototype/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/io-prompt-prototype/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
