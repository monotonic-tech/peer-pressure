use peer_pressure::Validate;

#[derive(Debug, PartialEq)]
struct RawScore(i32);
#[derive(Debug, PartialEq)]
struct Score(u32);

impl Validate for RawScore {
    type Context = ();
    type Output = Score;
    type Error = &'static str;

    fn validate_in_context(self, _: &()) -> Result<Score, Self::Error> {
        u32::try_from(self.0)
            .map(Score)
            .map_err(|_| "negative score")
    }
}

#[derive(Debug, PartialEq)]
struct RawLabel(&'static str);
#[derive(Debug, PartialEq)]
struct Label(&'static str);

impl Validate for RawLabel {
    type Context = ();
    type Output = Label;
    type Error = &'static str;

    fn validate_in_context(self, _: &()) -> Result<Label, Self::Error> {
        if self.0.is_empty() {
            Err("empty label")
        } else {
            Ok(Label(self.0))
        }
    }
}

#[test]
fn pair_validates_into_a_pair_of_outputs() {
    assert_eq!(
        (RawScore(10), RawLabel("hp")).validate(),
        Ok((Score(10), Label("hp")))
    );
}

#[test]
fn pair_propagates_the_first_element_error() {
    assert_eq!(
        (RawScore(-1), RawLabel("hp")).validate(),
        Err("negative score")
    );
}

#[test]
fn pair_propagates_a_later_element_error() {
    assert_eq!((RawScore(10), RawLabel("")).validate(), Err("empty label"));
}

#[test]
fn pair_short_circuits_before_validating_later_elements() {
    assert_eq!(
        (RawScore(-1), RawLabel("")).validate(),
        Err("negative score")
    );
}

#[test]
fn triple_preserves_element_order() {
    assert_eq!(
        (RawScore(1), RawLabel("a"), RawScore(2)).validate(),
        Ok((Score(1), Label("a"), Score(2)))
    );
}

#[test]
fn nested_tuples_validate_recursively() {
    assert_eq!(
        (RawScore(1), (RawLabel("a"), RawScore(2))).validate(),
        Ok((Score(1), (Label("a"), Score(2))))
    );
}

#[derive(Debug, PartialEq)]
struct Floor(i32);

#[derive(Debug, PartialEq)]
struct RawAbove(i32);
#[derive(Debug, PartialEq)]
struct Above(i32);

impl Validate for RawAbove {
    type Context = Floor;
    type Output = Above;
    type Error = &'static str;

    fn validate_in_context(self, floor: &Floor) -> Result<Above, Self::Error> {
        if self.0 > floor.0 {
            Ok(Above(self.0))
        } else {
            Err("below floor")
        }
    }
}

#[test]
fn tuple_threads_one_shared_context_into_every_element() {
    assert_eq!(
        (RawAbove(5), RawAbove(9)).validate_in_context(&Floor(3)),
        Ok((Above(5), Above(9)))
    );
    assert_eq!(
        (RawAbove(5), RawAbove(1)).validate_in_context(&Floor(3)),
        Err("below floor")
    );
}

#[test]
fn large_tuples_are_supported_up_to_the_documented_ceiling() {
    let raws = (
        RawScore(1),
        RawScore(2),
        RawScore(3),
        RawScore(4),
        RawScore(5),
        RawScore(6),
        RawScore(7),
        RawScore(8),
        RawScore(9),
        RawScore(10),
        RawScore(11),
        RawScore(12),
        RawScore(13),
        RawScore(14),
        RawScore(15),
        RawScore(16),
        RawScore(17),
        RawScore(18),
        RawScore(19),
        RawScore(20),
        RawScore(21),
        RawScore(22),
        RawScore(23),
        RawScore(24),
        RawScore(25),
        RawScore(26),
    );
    let validated = raws.validate().unwrap();
    assert_eq!(validated.0, Score(1));
    assert_eq!(validated.25, Score(26));
}
