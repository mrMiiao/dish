pub use core::fmt::*;

pub trait FormatNum = Display + Debug + LowerExp + UpperExp;
pub trait FormatInt = LowerHex + UpperHex + Binary + Octal;