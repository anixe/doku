/// This test checks an extra (and, frankly, a bit unrealistic) edge case when
/// we're given an untagged enum with unit (!) fields - in this case Serde
/// treats all variants as `null` (making the enum effectively impossible to
/// deserialize).
///
/// Unlikely to happen in practice, but worth testing.

#[derive(Doku)]
#[doku(untagged)]
pub enum Ty {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
}
