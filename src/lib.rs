//! _**dish**_  provides several additions to libcore.

#![no_std]
#![feature(type_alias_impl_trait, trait_alias, core_intrinsics)]
#![allow(non_snake_case, non_camel_case_types)]

pub mod iter;
pub mod num;
pub mod fmt;
pub mod mem;

pub mod prelude {
    pub use crate::iter::Iter;
    pub use crate::num::Number;
}