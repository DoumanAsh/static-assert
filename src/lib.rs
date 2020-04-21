//! Simple `static_assert` macro for compile time  assertions
//!
//! ## Usage
//!
//! ```rust
//! use static_assert::static_assert;
//!
//! static_assert!(1 == 1);
//! static_assert!(1 == 1, "Must be equal");
//! ```
//!
//! ```compile_fail
//! use static_assert::static_assert;
//!
//! static_assert!(0 == 1, "Must be equal"); //should fail
//! ```
//!
//! ## Nightly detection
//!
//! Currently the lib automatically detects compiler version in order to decide whether enable `const_if_match` to use `compile_error!` for a better read-ability.

#![no_std]
#![cfg_attr(feature = "nightly", feature(const_if_match))]

#[macro_export]
//If const expression evaluates to `true`, this macro has no effect.
//
//Otherwise a compile-time error is issued.
//
//If `nightly` is available, compile error is issued using `compile_error!` either with user supplied message, or automatically generated.
//Otherwise compilation fails with obscure error that indicates that expression evaluates to
//`false`
macro_rules! static_assert {
    ($exp:expr) => {
        $crate::static_assert!($exp, core::concat!("Static assertion failed: ", core::stringify!($exp)));
    };
    ($exp:expr, $msg:expr) => {
        #[deny(const_err)]
        #[allow(unused_must_use)]
        const _: () = {
            #[cfg(not(nightly))]
            {
                $exp as usize - 1usize;
            }
            #[cfg(nightly)]
            {
                if !$exp {
                    core::compile_error!($msg);
                }
            }

            ()
        };
    };
}
