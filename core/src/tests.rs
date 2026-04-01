extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;

use crate::{Valid, Validate, ValidateFrom, ops::IteratorExt};

#[derive(Debug, PartialEq)]
struct RawPositive(i32);

#[derive(Debug, PartialEq)]
struct PositiveNumber(i32);

impl Validate for RawPositive {
    type Context = ();
    type Output = PositiveNumber;
    type Error = &'static str;

    fn validate_in_context(self, _: &()) -> Result<PositiveNumber, &'static str> {
        if self.0 > 0 {
            Ok(PositiveNumber(self.0))
        } else {
            Err("not positive")
        }
    }
}

impl ValidateFrom<i32> for PositiveNumber {
    type Context = ();
    type Error = &'static str;

    fn validate_from_in_context(raw: i32, _: &()) -> Result<Self, &'static str> {
        if raw > 0 {
            Ok(PositiveNumber(raw))
        } else {
            Err("not positive")
        }
    }
}

#[derive(Debug, PartialEq)]
struct RawAboveThreshold1(i32);

#[derive(Debug, PartialEq)]
struct AboveThreshold1(i32);

struct Threshold(i32);

trait ThresholdT {
    fn accepts(&self, n: i32) -> bool;
}

impl ThresholdT for Threshold {
    fn accepts(&self, n: i32) -> bool {
        n >= self.0
    }
}

impl Validate for RawAboveThreshold1 {
    type Context = Threshold;
    type Output = AboveThreshold1;
    type Error = &'static str;

    fn validate_in_context(self, ctx: &Threshold) -> Result<AboveThreshold1, &'static str> {
        if self.0 >= ctx.0 {
            Ok(AboveThreshold1(self.0))
        } else {
            Err("below threshold")
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct AlreadyValid(u32);

impl Valid for AlreadyValid {}

#[derive(Debug, PartialEq)]
struct AboveThreshold2(i32);

struct RawAboveThreshold2(i32);

impl Validate for RawAboveThreshold2 {
    type Context = dyn ThresholdT;
    type Output = AboveThreshold2;
    type Error = &'static str;

    fn validate_in_context(self, ctx: &dyn ThresholdT) -> Result<AboveThreshold2, &'static str> {
        if ctx.accepts(self.0) {
            Ok(AboveThreshold2(self.0))
        } else {
            Err("rejected by policy")
        }
    }
}

impl ValidateFrom<i32> for AboveThreshold2 {
    type Context = dyn ThresholdT;
    type Error = &'static str;

    fn validate_from_in_context(raw: i32, ctx: &dyn ThresholdT) -> Result<Self, &'static str> {
        if ctx.accepts(raw) {
            Ok(AboveThreshold2(raw))
        } else {
            Err("rejected by policy")
        }
    }
}

mod validate_trait {
    use super::*;

    #[test]
    fn sugar_delegates_to_validate_in_context_for_unit_context() {
        assert_eq!(RawPositive(5).validate(), Ok(PositiveNumber(5)));
    }
}

mod validate_from_trait {
    use super::*;

    #[test]
    fn sugar_delegates_to_validate_from_in_context_for_unit_context() {
        assert_eq!(PositiveNumber::validate_from(5), Ok(PositiveNumber(5)));
    }
}

mod valid_blanket {
    use super::*;

    #[test]
    fn makes_validate_available() {
        let already = AlreadyValid(7);
        let out: Result<AlreadyValid, Infallible> = already.clone().validate();
        assert_eq!(out, Ok(already));
    }

    #[test]
    fn makes_validate_from_self_available() {
        let already = AlreadyValid(7);
        let out: Result<AlreadyValid, Infallible> = AlreadyValid::validate_from(already.clone());
        assert_eq!(out, Ok(already));
    }
}

mod unsized_context {
    use super::*;

    #[test]
    fn validate_accepts_dyn_trait_as_context() {
        let policy: &dyn ThresholdT = &Threshold(3);
        assert_eq!(
            RawAboveThreshold2(5).validate_in_context(policy),
            Ok(AboveThreshold2(5))
        );
    }

    #[test]
    fn validate_from_accepts_dyn_trait_as_context() {
        let policy: &dyn ThresholdT = &Threshold(3);
        assert_eq!(
            AboveThreshold2::validate_from_in_context(5, policy),
            Ok(AboveThreshold2(5))
        );
    }
}

mod validate_all {
    use super::*;

    #[test]
    fn collects_outputs_when_all_inputs_are_valid() {
        let raws = vec![RawPositive(1), RawPositive(2), RawPositive(3)];
        let out: Result<Vec<PositiveNumber>, _> = raws.validate_all();
        assert_eq!(
            out,
            Ok(vec![
                PositiveNumber(1),
                PositiveNumber(2),
                PositiveNumber(3)
            ])
        );
    }

    #[test]
    fn short_circuits_on_first_error() {
        let raws = vec![RawPositive(1), RawPositive(-1), RawPositive(3)];
        let out: Result<Vec<PositiveNumber>, _> = raws.validate_all();
        assert_eq!(out, Err("not positive"));
    }

    #[test]
    fn in_context_threads_context_into_validation() {
        let raws = vec![RawAboveThreshold1(5), RawAboveThreshold1(10)];
        let out: Result<Vec<AboveThreshold1>, _> = raws.validate_all_in_context(&Threshold(3));
        assert_eq!(out, Ok(vec![AboveThreshold1(5), AboveThreshold1(10)]));
    }
}

mod partition_validated {
    use super::*;

    #[test]
    fn splits_outputs_and_errors() {
        let raws = vec![
            RawPositive(1),
            RawPositive(-1),
            RawPositive(3),
            RawPositive(-2),
        ];
        let (oks, errs): (Vec<PositiveNumber>, Vec<&'static str>) = raws.partition_validated();
        assert_eq!(oks, vec![PositiveNumber(1), PositiveNumber(3)]);
        assert_eq!(errs, vec!["not positive", "not positive"]);
    }

    #[test]
    fn yields_empty_errors_when_all_inputs_are_valid() {
        let raws = vec![RawPositive(1), RawPositive(2)];
        let (oks, errs): (Vec<PositiveNumber>, Vec<&'static str>) = raws.partition_validated();
        assert_eq!(oks, vec![PositiveNumber(1), PositiveNumber(2)]);
        assert!(errs.is_empty());
    }

    #[test]
    fn in_context_splits_outputs_and_errors_using_context() {
        let raws = vec![
            RawAboveThreshold1(5),
            RawAboveThreshold1(2),
            RawAboveThreshold1(10),
        ];
        let (oks, errs): (Vec<AboveThreshold1>, Vec<&'static str>) =
            raws.partition_validated_in_context(&Threshold(3));
        assert_eq!(oks, vec![AboveThreshold1(5), AboveThreshold1(10)]);
        assert_eq!(errs, vec!["below threshold"]);
    }
}

/// Makse sure the derive works correctly inside this crate and its doctests.
#[cfg(feature = "derive")]
mod derive_within_own_crate {
    use crate::{Validate, derive::Valid};

    #[derive(Valid, Debug, PartialEq)]
    struct InternallyValid(u8);

    #[test]
    fn derive_resolves_the_crate_path_from_inside_peer_pressure() {
        assert_eq!(InternallyValid(3).validate(), Ok(InternallyValid(3)));
    }
}
