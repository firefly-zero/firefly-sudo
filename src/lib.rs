#![doc = include_str!("../README.md")]
#![no_std]
#![deny(
    clippy::pedantic,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::expect_used,
    clippy::unwrap_used
)]
#![allow(clippy::wildcard_imports)]
#![expect(clippy::cast_possible_truncation, clippy::iter_without_into_iter)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod sudo;
