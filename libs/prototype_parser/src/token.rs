#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Literal(&'a str),
    Interpolation(&'a str),
    SectionOpener(&'a str),
    SectionCloser(&'a str),
}