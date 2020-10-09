/// Enum to store the various types of errors that can cause parsing a TinyStr to fail.
#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    /// String is too large or too small to store as TinyStr.
    InvalidSize,
    /// String is empty.
    InvalidNull,
    /// String contains non-ASCII character(s).
    NonAscii,
}
