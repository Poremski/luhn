# luhncalc

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Luhn Calculator is a command-line utility to quickly validate a digit sequence using Luhn algorithm and calculate the next validation digit to be appended to the digit sequence to make whole sequence valid.

# How to use

Validate the following invalid digit sequence `1234567890`:

```bash
> luhncalc 1234567890

Sequence:       1234567890
Valid sequence: false
Next digit:     3
```

Validate the following valid digit sequence `12345678903`:

```bash
> luhncalc 12345678903

Sequence:       12345678903
Valid sequence: true
Next digit:     1
```

# License

This project is licensed under either of

- [the Apache License, Version 2.0](./LICENSE-APACHE)
- [the MIT license](./LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-20 license, shall be dual licensed as above, without any additional terms or conditions.

[1]: https://img.shields.io/crates/v/luhncalc.svg?style=flat-square
[2]: https://crates.io/crates/luhncalc
[3]: https://img.shields.io/travis/com/poremski/luhncalc/main?style=flat-square
[4]: https://travis-ci.com/poremski/luhncalc
[5]: https://img.shields.io/crates/d/luhncalc.svg?style=flat-square
[6]: https://crates.io/crates/luhncalc
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/luhncalc

[releases]: https://github.com/poremski/luhncalc/releases
