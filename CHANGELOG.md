# CHANGELOG

## Unreleased

- Add a trace runner ([#9](https://github.com/informalsystems/itf-rs/pull/9))

## v0.2.0

*November 21st, 2023*

- Deserialize ITF values into native Rust types with a custom deserializer
  instead of having to go through `Itf<A>` wrapper type.
  ([#6](https://github.com/informalsystems/itf-rs/pull/6))

## v0.1.2

*November 10th, 2023*

- Add `From<T> where T: From<BigInt>` instance for `ItfBigInt`

## v0.1.1

*November 10th, 2023*

- Add support for new `timestamp` field in meta section of ITF traces

## v0.1

*March 24th, 2023*

- Initial release

