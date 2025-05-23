#![allow(clippy::arithmetic_side_effects)]
//! Utilities for the [borsh] serialization format, version 0.9.
//!
//! This file is provided for backwards compatibility with types that still use
//! borsh 0.9, even though this crate canonically uses borsh 0.10.
//!
//! [borsh]: https://borsh.io/
use {
    crate::borsh::{
        impl_get_instance_packed_len, impl_get_packed_len_v0, impl_try_from_slice_unchecked,
    },
    borsh0_9::maybestd::io,
};

impl_get_packed_len_v0!(
    borsh0_9,
    #[deprecated(
        since = "1.17.0",
        note = "Please upgrade to Borsh 1.X and use `borsh1::get_packed_len` instead"
    )]
);
impl_try_from_slice_unchecked!(
    borsh0_9,
    io,
    #[deprecated(
        since = "1.17.0",
        note = "Please upgrade to Borsh 1.X and use `borsh1::try_from_slice_unchecked` instead"
    )]
);
impl_get_instance_packed_len!(
    borsh0_9,
    io,
    #[deprecated(
        since = "1.17.0",
        note = "Please upgrade to Borsh 1.X and use `borsh1::get_instance_packed_len` instead"
    )]
);

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use {crate::borsh::impl_tests, borsh0_9::maybestd::io};
    impl_tests!(borsh0_9, io);
}