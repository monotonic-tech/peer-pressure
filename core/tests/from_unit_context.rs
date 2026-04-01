//! Tests for context-free validation extensions

use peer_pressure::{Validate, ValidateFrom};

struct Limit(u32);

impl From<()> for Limit {
    fn from((): ()) -> Self {
        Limit(10)
    }
}

#[derive(Debug, PartialEq)]
struct Small(u32);

struct RawSmall(u32);

fn check(raw: u32, limit: &Limit) -> Result<Small, &'static str> {
    if raw <= limit.0 {
        Ok(Small(raw))
    } else {
        Err("too big")
    }
}

impl Validate for RawSmall {
    type Context = Limit;
    type Output = Small;
    type Error = &'static str;

    fn validate_in_context(self, limit: &Limit) -> Result<Small, &'static str> {
        check(self.0, limit)
    }
}

impl ValidateFrom<u32> for Small {
    type Context = Limit;
    type Error = &'static str;

    fn validate_from_in_context(raw: u32, limit: &Limit) -> Result<Self, &'static str> {
        check(raw, limit)
    }
}

#[test]
fn validate_uses_the_context_built_from_unit() {
    assert_eq!(RawSmall(11).validate(), Err("too big"));
}

#[test]
fn validate_from_uses_the_context_built_from_unit() {
    assert_eq!(Small::validate_from(11), Err("too big"));
}

#[cfg(feature = "async")]
mod async_sugar {
    use super::*;
    use futures::executor::block_on;
    use peer_pressure::{IteratorExt, Resolve, ResolveFrom};

    impl Resolve for RawSmall {
        type Context = Limit;
        type Output = Small;
        type Error = &'static str;

        async fn resolve_in_context(self, limit: &Limit) -> Result<Small, &'static str> {
            check(self.0, limit)
        }
    }

    impl ResolveFrom<u32> for Small {
        type Context = Limit;
        type Error = &'static str;

        async fn resolve_from_in_context(raw: u32, limit: &Limit) -> Result<Self, &'static str> {
            check(raw, limit)
        }
    }

    #[test]
    fn resolve_uses_the_context_built_from_unit() {
        assert_eq!(block_on(RawSmall(11).resolve()), Err("too big"));
    }

    #[test]
    fn resolve_from_uses_the_context_built_from_unit() {
        assert_eq!(block_on(Small::resolve_from(11)), Err("too big"));
    }

    #[test]
    fn partition_resolved_uses_the_context_built_from_unit() {
        let (oks, errs): (Vec<Small>, Vec<&'static str>) =
            block_on([RawSmall(1), RawSmall(11)].partition_resolved());
        assert_eq!((oks, errs), (vec![Small(1)], vec!["too big"]));
    }
}
