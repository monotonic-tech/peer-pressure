#![cfg(feature = "async")]

use futures::executor::block_on;
use peer_pressure::{Resolve, ResolveFrom};

#[derive(Debug, PartialEq)]
struct RawId(u64);
#[derive(Debug, PartialEq)]
struct VerifiedId(u64);

impl Resolve for RawId {
    type Context = ();
    type Output = VerifiedId;
    type Error = &'static str;
    async fn resolve_in_context(self, _: &()) -> Result<VerifiedId, &'static str> {
        Ok(VerifiedId(self.0))
    }
}

#[test]
fn resolve_sugar_applies_for_unit_context() {
    assert_eq!(block_on(RawId(42).resolve()), Ok(VerifiedId(42)));
}

#[test]
fn resolve_in_context_drives_future_to_completion() {
    assert_eq!(
        block_on(RawId(7).resolve_in_context(&())),
        Ok(VerifiedId(7))
    );
}

#[derive(Debug, PartialEq)]
struct Handle(String);

impl ResolveFrom<String> for Handle {
    type Context = ();
    type Error = &'static str;
    async fn resolve_from_in_context(raw: String, _: &()) -> Result<Self, &'static str> {
        if raw.starts_with('@') {
            Ok(Handle(raw))
        } else {
            Err("missing @")
        }
    }
}

#[test]
fn resolve_from_sugar_applies_for_unit_context() {
    assert_eq!(
        block_on(Handle::resolve_from(String::from("@alice"))),
        Ok(Handle("@alice".into()))
    );
}

#[test]
fn resolve_from_in_context_reports_error() {
    assert_eq!(
        block_on(Handle::resolve_from_in_context(String::from("alice"), &())),
        Err("missing @")
    );
}
