# peer-pressure

<img src="https://raw.githubusercontent.com/monotonic-tech/peer-pressure/main/gossips.jpeg"
     alt="A cropped fragment of the 1948 painting 'The Gossips', by Norman Rockwell, containing a chain of gossipping people. It's a deep metaphor."/>

Peer pressure is a stupidly simple parse-don't-validation micro-framework for Rust types that seek to conform to society's expectations. You know, rules like these:

- A file name must not contain a `/` slash,
- The number of items in cart must be non-negative,
- Password must contain exactly 3 emojis, a prime number, your mother's maiden name in reverse, and cannot include any letter found in the word "password",

..yada yada yada - `peer-pressure` gives you a small, cohesive set of traits and types to define your validation logic in a principled way.

## Principles

This crate is highly opinionated and deliberately constraining in accordance with the following beliefs:

0. Validation should happen at system boundary and no constructed value should still need to be validated.
   [In a perfect world, this crate would not have to exist](https://www.youtube.com/watch?v=Kl3H4vMqYNo).
1. Validation should be functionally pure
2. Validation should consume the validated value
3. Validity should be expressed at the type level
4. Validation context should be declared explicitly
5. Types should have a single notion of validity

## Quick Start

Add `peer-pressure` to your dependencies and implement `Validate` for any
raw type that needs to be turned into a validated shape:

```rust
use peer_pressure::Validate;

struct RawEmail(String);
struct ValidEmail(String);

impl Validate for RawEmail {
    type Context = ();
    type Output = ValidEmail;
    type Error = String;

    fn validate_in_context(self, _: &()) -> Result<ValidEmail, String> {
        if self.0.contains('@') {
            Ok(ValidEmail(self.0))
        } else {
            Err(format!("{} is not a valid email address", self.0))
        }
    }
}

// `validate()` is a syntax sugar for types with context-free validation:
let valid_email = RawEmail("alice@example.com".into()).validate().unwrap();
```

For types whose validation require context, eg. a config or a lookup table, define a `Context` type other than `()`:

```rust
use peer_pressure::Validate;
use std::collections::HashSet;

struct Text(String);
#[derive(Debug)]
struct ValidText(String);

type Alphabet = HashSet<char>;

impl Validate for Text {
    type Context = Alphabet;
    type Output = ValidText;
    type Error = String;

    fn validate_in_context(self, alphabet: &Alphabet) -> Result<ValidText, String> {
        if self.0.chars().all(|ch| alphabet.contains(&ch)) {
            Ok(ValidText(self.0))
        } else {
            Err(format!("{} is not a valid text in {alphabet:?}", self.0))
        }
    }
}

let alphabet = Alphabet::from(['a', 'b', 'c']);

let valid_text = Text("abba".into()).validate_in_context(&alphabet).unwrap();
let validation_error = Text("not a valid text".into()).validate_in_context(&alphabet).unwrap_err();
```

## Valid types

Types whose values are considered already valid can implement the `Valid` marker trait.

``` rust
use peer_pressure::Valid;

/// I make no sense at all as a type!
pub struct ValidU64(u64);

impl Valid for ValidU64 {}
```

``` rust
# #[cfg(feature = "derive")]
# mod example {
use peer_pressure::derive::Valid;

#[derive(Valid)]
pub struct ValidU128(u128);
# }
```

This gives two benefits:
1. it auto-derives `Validate` for the type, preventing anyone from implementing a spurious one,
2. it can be put as a bound in generic code that assumes valid input.

The `Validate` trait does not impose that the `Output` type implements `Valid`, but it's a good practice.

## Tuples

All traits are implemented for tuples up to 26 elements, so a tuple of inputs validates into a tuple of outputs:

```rust
use peer_pressure::ValidateFrom;

struct Name(String);
struct Age(u8);

impl ValidateFrom<&str> for Name {
    type Context = ();
    type Error = &'static str;
    fn validate_from_in_context(raw: &str, _: &()) -> Result<Self, Self::Error> {
        if raw.is_empty() { Err("empty name") } else { Ok(Name(raw.to_string())) }
    }
}

impl ValidateFrom<i32> for Age {
    type Context = ();
    type Error = &'static str;
    fn validate_from_in_context(raw: i32, _: &()) -> Result<Self, Self::Error> {
        u8::try_from(raw).map(Age).map_err(|_| "age out of range")
    }
}

let (name, age) = <(Name, Age)>::validate_from(("ada", 36)).unwrap();
```

One constraint for this is that all elements must agree on one `Context` type and one `Error` type.

## Async

In a perfect world, all validation would be synchronous and pure. But unlike types in computer systems,
the world doesn't cave to any pressure, and sometimes a "validation" step needs to be async. A typical
case is the need for a database lookup or a call to another service. To account for this, peer-pressure
provides `Resolve` and `ResolveFrom` traits analogous to `Validate` and `ValidateFrom`. 

## Features

- `derive`: provides `#[derive(Valid)]` for the `Valid` trait.
- `async`: enables the async `Resolve` trait family.

## License

[Apache License, Version 2.0][LICENSE-APACHE] or [MIT license][LICENSE-MIT].

Any contribution submitted to this crate by you shall be dual licensed as above,
without any additional terms or conditions. You've been warned.

---

[LICENSE-APACHE]: https://github.com/monotonic-tech/peer-pressure/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/monotonic-tech/peer-pressure/blob/main/LICENSE-MIT
