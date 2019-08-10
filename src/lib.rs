mod helpers;
mod tinystr4;
mod tinystr8;

pub use tinystr4::TinyStr4;
pub use tinystr8::TinyStr8;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidSize,
    InvalidNull,
    NonAscii,
}
