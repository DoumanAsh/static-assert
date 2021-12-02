//! Simple `static_assert` macro for compile time  assertions
//!
//! Uses `const_panic` within `const` variable to produce compile error hence only usable in `const` context.
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
//! static_assert!(0 == 1, "Must be equal"); //should fail
//! ```
//!
//! ```compile_fail
//! use sa::static_assert;
//!
//! static_assert!(0 == 1); //should fail
//! ```
#![no_std]

#[macro_export]
///If expression evaluates to `true`, this macro has no effect.
///
///Otherwise a compile-time error is issued via panic.
///
///## Arguments:
///
///- `exp` - expression to evaluate. Result of expression must be `bool`
///- `msg` - optional string literal to add to the error message.
macro_rules! static_assert {
    ($exp:expr) => {
        #[deny(const_err)]
        #[allow(unused_must_use)]
        const _: () = {
            if !($exp) {
                core::panic!(core::concat!("Static assertion '", core::stringify!($exp), "' failed"));
            }

            ()
        };
    };
    ($exp:expr, $msg:literal) => {
        #[deny(const_err)]
        #[allow(unused_must_use)]
        const _: () = {
            if !($exp) {
                core::panic!(core::concat!("Static assertion '", core::stringify!($exp), "' failed: ", $msg));
            }

            ()
        };
    };
}
