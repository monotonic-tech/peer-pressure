use peer_pressure::{IteratorExt, Validate};

#[derive(Debug, PartialEq)]
struct RawNumber(i32);
#[derive(Debug, PartialEq)]
struct PositiveNumber(i32);

impl Validate for RawNumber {
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

#[test]
fn validate_all_collects_successes() {
    let out: Result<Vec<PositiveNumber>, _> =
        vec![RawNumber(1), RawNumber(2), RawNumber(3)].validate_all();
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
fn validate_all_short_circuits_on_first_error() {
    let out: Result<Vec<PositiveNumber>, _> =
        vec![RawNumber(1), RawNumber(-1), RawNumber(3)].validate_all();
    assert_eq!(out, Err("not positive"));
}

#[test]
fn partition_validated_splits() {
    let raws = vec![RawNumber(1), RawNumber(-1), RawNumber(3), RawNumber(-2)];
    let (oks, errs): (Vec<PositiveNumber>, Vec<&'static str>) = raws.partition_validated();
    assert_eq!(oks, vec![PositiveNumber(1), PositiveNumber(3)]);
    assert_eq!(errs, vec!["not positive", "not positive"]);
}

#[cfg(feature = "async")]
mod async_tests {
    use super::*;
    use futures::executor::block_on;
    use peer_pressure::Resolve;

    impl Resolve for RawNumber {
        type Context = ();
        type Output = PositiveNumber;
        type Error = &'static str;

        async fn resolve_in_context(self, _: &()) -> Result<Self::Output, Self::Error> {
            self.validate()
        }
    }

    #[test]
    fn all_resolved_collects_successes() {
        let out: Result<Vec<PositiveNumber>, _> =
            block_on(vec![RawNumber(1), RawNumber(2), RawNumber(3)].resolve_all());
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
    fn all_resolved_short_circuits_on_first_error() {
        let out: Result<Vec<PositiveNumber>, _> =
            block_on(vec![RawNumber(1), RawNumber(-1), RawNumber(3)].resolve_all());
        assert_eq!(out, Err("not positive"));
    }

    #[test]
    fn partition_resolved_splits() {
        let raws = vec![RawNumber(1), RawNumber(-1), RawNumber(3), RawNumber(-2)];
        let (oks, errs): (Vec<PositiveNumber>, Vec<&'static str>) =
            block_on(raws.partition_resolved());
        assert_eq!(oks, vec![PositiveNumber(1), PositiveNumber(3)]);
        assert_eq!(errs, vec!["not positive", "not positive"]);
    }
}
