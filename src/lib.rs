//! Simple `static_assert` macro for compile time  assertions
//!
//! ## Usage
//!
//! ```rust
//! use sa::static_assert;
//!
//! static_assert!(1 == 1);
//! static_assert!(1 == 1, "Must be equal");
//! ```
//!
//! ```compile_fail
//! use sa::static_assert;
//!
//!
//! static_assert!(0 == 1, "Must be equal"); //should fail
//! ```

#![no_std]

#[macro_export]
///If const expression evaluates to `true`, this macro has no effect.
///
///Otherwise a compile-time error is issued.
macro_rules! static_assert {
    ($exp:expr) => {
        $crate::static_assert!($exp, core::concat!("Static assertion failed: ", core::stringify!($exp)));
    };
    ($exp:expr, $msg:expr) => {
        #[deny(const_err)]
        #[allow(unused_must_use)]
        const _: () = {
            $exp as usize - 1usize;

            ()
        };
    };
}
