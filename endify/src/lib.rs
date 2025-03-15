//! # endify
//!
//! Effortlessly convert structs between endianess-formats.
//!
//! ## Usage
//! ```rust
//! use endify::Endify;
//!
//! #[repr(C)]
//! #[derive(Debug, Endify)]
//! struct MyStruct {
//!     a: u32,
//!     b: u16,
//!     c: u8,
//! }
//!
//! # fn read_from_disk() -> MyStruct {
//! #   MyStruct { a: 30u32.to_be(), b: 10u16.to_be(), c: 3u8.to_be() }
//! # }
//!
//! fn main() {
//!     // stored on disk as `little-endian` format.
//!     let my_struct = read_from_disk();
//!
//!     // convert all fields to `native-endian` format.
//!     let my_struct_native = my_struct.from_le();
//! }
//! ```

#![no_std]
#![allow(clippy::wrong_self_convention)]
pub use endify_derive::Endify;

/// A trait for converting between endianess-formts effortlessly.
pub trait Endify {
    /// Converts a struct to little-endian format.
    fn to_le(self) -> Self;
    /// Converts a struct to big-endian format.
    fn to_be(self) -> Self;
    /// Converts a struct stored in little-endian format to native-endian format.
    fn from_le(self) -> Self;
    /// Converts a struct stored in big-endian format to native-endian format.
    fn from_be(self) -> Self;
}
