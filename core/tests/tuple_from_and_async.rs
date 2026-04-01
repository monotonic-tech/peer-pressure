use peer_pressure::ValidateFrom;

#[derive(Debug, PartialEq)]
struct Name(String);
#[derive(Debug, PartialEq)]
struct Age(u8);

impl ValidateFrom<&str> for Name {
    type Context = ();
    type Error = &'static str;

    fn validate_from_in_context(raw: &str, _: &()) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err("empty name")
        } else {
            Ok(Name(raw.to_string()))
        }
    }
}

impl ValidateFrom<i32> for Age {
    type Context = ();
    type Error = &'static str;

    fn validate_from_in_context(raw: i32, _: &()) -> Result<Self, Self::Error> {
        u8::try_from(raw).map(Age).map_err(|_| "age out of range")
    }
}

#[test]
fn validate_from_builds_a_pair_from_a_pair_of_raws() {
    let got: Result<(Name, Age), _> = <(Name, Age)>::validate_from(("ada", 36));
    assert_eq!(got, Ok((Name("ada".to_string()), Age(36))));
}

#[test]
fn validate_from_propagates_the_first_element_error() {
    let got: Result<(Name, Age), _> = <(Name, Age)>::validate_from(("", 36));
    assert_eq!(got, Err("empty name"));
}

#[test]
fn validate_from_propagates_a_later_element_error() {
    let got: Result<(Name, Age), _> = <(Name, Age)>::validate_from(("ada", 9000));
    assert_eq!(got, Err("age out of range"));
}

#[test]
fn validate_from_threads_raws_positionally_in_a_triple() {
    let got: Result<(Name, Age, Name), _> =
        <(Name, Age, Name)>::validate_from(("ada", 36, "grace"));
    assert_eq!(
        got,
        Ok((Name("ada".to_string()), Age(36), Name("grace".to_string())))
    );
}

#[cfg(feature = "async")]
mod async_tuples {
    use futures::executor::block_on;
    use peer_pressure::{Resolve, ResolveFrom};

    #[derive(Debug, PartialEq)]
    struct RawPort(i32);
    #[derive(Debug, PartialEq)]
    struct Port(u16);

    impl Resolve for RawPort {
        type Context = ();
        type Output = Port;
        type Error = &'static str;

        async fn resolve_in_context(self, _: &()) -> Result<Port, Self::Error> {
            u16::try_from(self.0).map(Port).map_err(|_| "bad port")
        }
    }

    #[derive(Debug, PartialEq)]
    struct Host(String);

    impl ResolveFrom<&str> for Host {
        type Context = ();
        type Error = &'static str;

        async fn resolve_from_in_context(raw: &str, _: &()) -> Result<Self, Self::Error> {
            if raw.contains('.') {
                Ok(Host(raw.to_string()))
            } else {
                Err("not a hostname")
            }
        }
    }

    #[test]
    fn resolve_awaits_every_element_of_a_tuple() {
        let got = block_on((RawPort(80), RawPort(443)).resolve());
        assert_eq!(got, Ok((Port(80), Port(443))));
    }

    #[test]
    fn resolve_propagates_a_later_element_error() {
        let got = block_on((RawPort(80), RawPort(-1)).resolve());
        assert_eq!(got, Err("bad port"));
    }

    #[test]
    fn resolve_from_builds_a_pair_from_a_pair_of_raws() {
        let got = block_on(<(Host, Host)>::resolve_from(("a.example", "b.example")));
        assert_eq!(
            got,
            Ok((Host("a.example".to_string()), Host("b.example".to_string())))
        );
    }

    #[test]
    fn resolve_from_propagates_the_first_element_error() {
        let got = block_on(<(Host, Host)>::resolve_from(("nope", "b.example")));
        assert_eq!(got, Err("not a hostname"));
    }
}
