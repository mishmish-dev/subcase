//! # Share parts of your test case intuitively
//!
//! ## What is a subcase?
//!
//! *Sections*, or *subcases* are a cool feature of unit testing frameworks,
//! such as (awesome) C++ libraries [Catch2](https://github.com/catchorg/Catch2)
//! and [doctest](https://github.com/doctest/doctest).
//! Subcases provide an easy way to share code between tests,
//! like fixtures do, but without needing to move setup and teardown code
//! outside of your tests' meat, without hassles of object orientation.
//!
//! How do they work? Subcases allow you to fork function execution
//! to into different paths which will have common code in the places
//! you want them to.
//!
//! Let's look at an example.
//! ```
//! use subcase::with_subcases;
//! with_subcases! {
//!     #[test]
//!     fn my_test_case() {
//!         let mut v = vec![1,2,3];
//!         
//!         subcase! {
//!             v.push(9);
//!             assert_eq!(v[3], 9);
//!         }
//!         subcase! {
//!             v.clear();
//!             assert!(v.is_empty());
//!             for _i in 0..4 { v.push(1); }
//!         }
//!         
//!         assert_eq!(v.len(), 4);
//!         assert!(v.capacity() >= 4);
//!     }
//! }
//! ```
//! `my_test_case`'s body will be executed twice, first time
//! with first `subcase!{...}` block, ignoring the second,
//! and vice versa.
//!
//! That's not all! Subcases can be nested!
//! ```
//! use subcase::with_subcases;
//! with_subcases! {
//!     #[test]
//!     fn my_tremendous_test_case() {
//!         let mut v = vec![1,2,3];   
//!         subcase! {
//!             v.push(9);
//!         }
//!         subcase! {
//!             v.clear();
//!             v.push(100);
//!     
//!             subcase! {
//!                 for _i in 0..5 { v.push(1); }
//!                 assert_eq!(v.len(), 5);
//!             }
//!             subcase! {
//!                v.extend_from_slice(&[4,5,6,7,8]);
//!             }
//!             assert_eq!(v.len(), 6);
//!     
//!             v.pop();
//!             v.pop();
//!         }
//!         assert_eq!(v.len(), 4);
//!     }
//! }
//! ```
//! Test function body is executed 3 times: once
//! for each of leaf subcases (i.e. not containing more nested subcases),
//! while the big parent subcase is entered twice.
//!
//! You can write only one subcase or no subcases at all, function
//! will run as usual.
//!
//! ## Other oprions?
//!
//! Indeed, there are already a few crates that implement the concept
//! of subcases:
//! + [rust-catch](https://github.com/guydunton/rust-catch)
//! + [crossroads](https://crates.io/crates/crossroads)
//!
//! What distinguishes subcase crate from each of them, is that
//! subcase only uses lightweight declarative (i.e. `macro_rules!`)
//! macros and has zero dependencies. Also, `with_subcases` macro stuffs
//! all execution paths inside one function, instead of generating
//! many. These making it very easy on Rust compiler, in comparison
//! to the mentioned crates.
//!
//! (I will provide actual benchmarks in the future.)
//!
//! ## Limitations
//!
//! One technical consequence of how the crate was
//! implemented is that subcases from one test function can't run
//! in parallel. This may or may not slow down your tests' execution.
//! If you have a lot of fine-grained test cases, you should be fine.
//!
//! Also, as different branches of evaluation are switched at runtime,
//! you possibly can trigger borrow checker.
//!
//! ## License
//!
//! Licensed under MIT License.

#![deny(missing_docs)]

/// Allows you to change name for the inner subcase nacro
/// by defining your own version of [`with_subcases!`].
/// You can add attributes and documentation to
/// the produced outer macro.
#[macro_export]
macro_rules! def_custom_macro {
    (
        $(#[$meta:meta])*
        $name:ident($custom_subcase:ident)
    ) => {
        $crate::__detail_macro! (@def_custom_macro $name $custom_subcase [$] $($meta)*);
    };
}

def_custom_macro! {
    /// Allows you to fork function execution and
    /// run different flow paths. For usage, consult the crade documentation.
    #[macro_export]
    with_subcases(subcase)
}

def_custom_macro! {
    /// The Catch2 flavour of [`with_subcases!`]. Use `section!`
    /// for the inner macro
    #[macro_export]
    with_sections(section)
}

/// Defines [`ErrTestable`] trait for checking returned
/// values for errors, and implements it for
/// [`Result<T, E>`], [`Option<T>`] and [`()`](unit).
pub mod err_testable;
pub use err_testable::ErrTestable;

#[doc(hidden)]
pub mod __detail;
