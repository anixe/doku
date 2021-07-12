#[derive(Doku)]
pub enum Ty {
    FirstVariant,

    #[doku(rename = "second")]
    SecondVariant,

    /// This is third variant
    #[doku(rename = "third")]
    ThirdVariant,

    /// This is fourth variant
    FourthVariant {
        /// This is fourth variant's first field
        a: String,
    },
}
