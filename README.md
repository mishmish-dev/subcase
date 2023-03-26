[![crates.io](https://img.shields.io/crates/v/subcase?style=for-the-badge&color=blue)](https://crates.io/crates/subcase)

<!-- cargo-rdme start -->

# Intuitive way to deduplicate your tests code

## What is a subcase?

*Sections*, or *subcases* are a cool feature of unit testing frameworks,
such as (awesome) C++ libraries [Catch2](https://github.com/catchorg/Catch2)
and [doctest](https://github.com/doctest/doctest).
Subcases provide an easy way to share code between tests,
like fixtures do, but without needing to move setup and teardown code
outside of your tests' meat, without hassles of object orientation.

How do they work? Subcases allow you to fork function execution
to into different paths which will have common code in the places
you want them to.

Let's look at an example.
```rust
use subcase::with_subcases;
with_subcases! {
    #[test]
    fn my_test_case() {
        let mut v = vec![1,2,3];
        
        subcase! {{
            v.push(9);
            assert_eq!(v.last().unwrap().clone(), 9);
        }}
        subcase! {{
            v.clear();
            assert!(v.is_empty());
            for _i in 0..4 { v.push(1); }
        }}
        
        assert_eq!(v.len(), 4);
        assert!(v.capacity() >= 4);
    }
}
```
`my_test_case`'s body will be executed twice, first time
with first `subcase!{{...}}` block, ignoring the second,
and vice versa.

That's not all! Subcases can be nested!
```rust
let mut v = vec![1,2,3];   
subcase! {{
    v.push(9);
}}
subcase! {{
    v.clear();

    subcase! {{
        for _i in 0..5 { v.push(1); }
        assert_eq!(v.len(), 5);
    }}
   
    v.push(100);
   
    subcase! {{
       v.extend_from_slice(&[4,5,6,7,8]);
    }}
    assert_eq!(v.len(), 6);

    v.pop();
    v.pop();
}}
assert_eq!(v.len(), 4);
```
In this example, test function body is executed 3 times: once
for each of leaf subcases (i.e. not containing more nested subcases),
while the big parent subcase is entered twice.

You can write only one subcase or no subcases at all --- function
will run as usual.

## Other oprions?

Indeed, there are already a few crates that implement the concept
of subcases:
+ [rust-catch](https://crates.io/crates/rust-catch)
+ [rye](https://crates.io/crates/rye)
+ [crossroads](https://crates.io/crates/crossroads)

What distinguishes subcase crate from each of them, is that
subcase only uses lightweight declarative (i.e. `macro_rules!`)
macros and has zero dependencies. Also, `with_subcases` macro stuffs
all execution paths inside one function, instead of generating
many. These making it very easy on Rust compiler, in comparison
to the mentioned crates.

(I will provide actual benchmarks in the future.)

## Limitations

Probably most of these limitations will be (partially) lifted in
the future, stay tuned.

+ As of current version, Rust builtin testing framework cannot help you
know what exact path of execution has failed. Also, as different
branches of evaluation are switched at runtime, you possibly can
trigger borrow checker.

+ Only `()`-returning functions are supported.

+ You must use double pair of braces with inner `subcase!` macro.

+ You cannot rename the inner `subcase!` macro.

## License

Licensed under MIT License.

<!-- cargo-rdme end -->
