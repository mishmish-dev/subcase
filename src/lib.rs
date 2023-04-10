//! # Share parts of your test case intuitively
//!
//! ## What is a subcase?
//!
//! _Sections_, or _subcases_ are a cool feature of unit testing frameworks,
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
//! ```rust
//! use subcase::with_subcases;
//! with_subcases! {
//!     #[test]
//!     fn my_test_case() {
//!         let mut v = vec![1,2,3];
//!         
//!         subcase! { ~"single push"
//!             v.push(9);
//!             assert_eq!(v[3], 9);
//!         }
//!         subcase! { ~"clear then push"
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
//! ```rust
//! use subcase::with_subcases;
//! with_subcases! {
//!     #[test]
//!     fn my_tremendous_test_case() {
//!         let mut v = vec![1,2,3];   
//!         subcase! { ~"single push"
//!             v.push(9);
//!         }
//!         subcase! { ~"clear, push, pop"
//!             v.clear();
//!             v.push(100);
//!     
//!             subcase! { ~"push in for loop"
//!                 for _i in 0..5 { v.push(1); }
//!             }
//!             subcase! { ~"extend from slice"
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
//! ## Technical approach and limitations
//!
//! Indeed, there are already a few crates that implement the concept
//! of subcases:
//! - [rust-catch](https://github.com/guydunton/rust-catch)
//! - [crossroads](https://crates.io/crates/crossroads)
//!
//! What distinguishes subcase crate from each of them, is that
//! subcase only uses lightweight declarative (i.e. `macro_rules!`)
//! macros and has zero dependencies. Also, `with_subcases` macro stuffs
//! all execution paths inside one function, instead of generating
//! many. These making it very easy on Rust compiler, in comparison
//! to the mentioned crates.
//! 
//! In `subcase`'s approach, subcases discovery and switching between them
//! happens serially at runtime.
//! 
//! One consequence of this is that different branches of a test case
//! can't run in parallel. This may or may not slow your tests down.
//! If you have a lot of fine-grained test cases, you should be fine.
//! 
//! Another consequence is that you generally cannot resume a test case
//! when one of the execution paths failed. If it failed with a panic,
//! `subcase` will report what chain of subcases caused that.
//! 
//! ## Changelog
//! 
//! You can read the changelog [here][changelog]. It follows
//! [Common Changelog][common-changelog] style guide and is written
//! with the help of [hallmark tool][hallmark]. 
//!
//! ## License
//!
//! Licensed under [MIT License][license].
//! 
//! [changelog]: https://github.com/mishmish-dev/subcase/blob/main/CHANGELOG.md
//! [common-changelog]: https://common-changelog.org
//! [hallmark]: https://github.com/vweevers/hallmark
//! [license]: https://github.com/mishmish-dev/subcase/blob/main/LICENSE.txt

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
    /// run different flow paths. For usage, consult the crate documentation.
    #[macro_export]
    with_subcases(subcase)
}

def_custom_macro! {
    /// The Catch2 flavour of [`with_subcases!`]. Uses `section!`
    /// for the inner macro.
    #[macro_export]
    with_sections(section)
}

#[doc(hidden)]
pub mod __detail;
