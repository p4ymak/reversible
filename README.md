# `Reversible<T>`

[![Latest version](https://img.shields.io/crates/v/reversible.svg)](https://crates.io/crates/reversible)

A generic wrapper struct that provides reversible editing capabilities for values of type `T: Default + Debug + Clone`.

## Features
- Tracks changes separately from the original data
- Only clones the original data when modifications begin (lazy cloning)
- Optional Serde support for serialization/deserialization (with `serde` feature)
- Implements `AsRef<T>` and `AsMut<T>` for transparent access to the original and changed data.

## Example

```rust
let mut rev = Reversible::from(4);
assert_eq!(rev.as_mut(), &mut 4);

*rev.as_mut() = 13;
assert_eq!(rev.as_ref(), &4);
assert_eq!(rev.as_mut(), &mut 13);

rev.save();
assert_eq!(rev.as_ref(), &13);

*rev.as_mut() = 4;
assert_eq!(rev.as_ref(), &13);
assert_eq!(rev.as_mut(), &mut 4);

rev.revert();
assert_eq!(rev.as_mut(), &mut 13);
```
