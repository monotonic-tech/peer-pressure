#![cfg(feature = "derive")]

use core::marker::PhantomData;
use peer_pressure::{Valid, Validate, ValidateFrom, derive::Valid as DeriveValid};

#[derive(DeriveValid, Debug, PartialEq)]
struct Port(u16);

#[test]
fn derived_type_satisfies_the_valid_bound() {
    fn assert_valid<T: Valid>() {}
    assert_valid::<Port>();
}

#[test]
fn derived_type_validates_as_itself() {
    assert_eq!(Port(443).validate(), Ok(Port(443)));
}

#[test]
fn derived_type_validates_from_itself() {
    assert_eq!(Port::validate_from(Port(80)), Ok(Port(80)));
}

#[test]
fn derived_validation_is_infallible() {
    let outcome: Result<Port, core::convert::Infallible> = Port(1).validate();
    assert!(outcome.is_ok());
}

#[derive(DeriveValid, Debug, PartialEq)]
struct Tagged<T> {
    value: u32,
    tag: PhantomData<T>,
}

#[test]
fn derive_carries_generics_over_to_the_impl() {
    let tagged = Tagged::<String> {
        value: 7,
        tag: PhantomData,
    };
    assert_eq!(
        tagged.validate(),
        Ok(Tagged::<String> {
            value: 7,
            tag: PhantomData
        })
    );
}

#[derive(DeriveValid, Debug, PartialEq)]
struct Bounded<T: Clone>(T)
where
    T: PartialEq;

#[test]
fn derive_preserves_a_where_clause() {
    assert_eq!(Bounded(5u8).validate(), Ok(Bounded(5u8)));
}

#[derive(DeriveValid, Debug, PartialEq)]
enum Mode {
    Fast,
    Slow,
}

#[test]
fn derive_applies_to_enums() {
    assert_eq!(Mode::Fast.validate(), Ok(Mode::Fast));
}

#[test]
fn derived_types_compose_inside_a_tuple() {
    assert_eq!(
        (Port(80), Mode::Slow).validate(),
        Ok((Port(80), Mode::Slow))
    );
}
