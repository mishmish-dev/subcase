#![allow(unused_macros)]

use subcase::with_subcases;

#[derive(Debug)]
struct ER {}

with_subcases! {
    #[test]
    fn my_test_case() {
        let mut v = vec![1,2,3];

        subcase! {
            v.push(9);
            assert_eq!(v.last().unwrap().clone(), 9);
        }
        subcase! {
            v.clear();
            assert!(v.is_empty());
            for _i in 0..4 { v.push(1); }
        }

        assert_eq!(v.len(), 4);
        assert!(v.capacity() >= 4);
    }

    #[test]
    fn my_tremendous_test_case() {
        let mut v = vec![1,2,3];

        subcase! {
            v.push(9);
        }
        subcase! {
            v.clear();
            assert!(v.is_empty());

            subcase! {
               for _i in 0..5 { v.push(1); }
               assert_eq!(v.len(), 5);
            }

            v.push(100);

            subcase! {
               v.extend_from_slice(&[4,5,6,7,8]);
            }

            assert_eq!(v.len(), 6);
            v.pop();
            v.pop();
        }

        assert_eq!(v.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_two() {
        let mut v = vec![1,2,3];

        subcase! {
            v.push(4);
            v.push(4);
        }
        subcase! {
            v.push(5);
        }

        assert_eq!(v.len(), 5);
    }

    #[test]
    fn returning_ok() -> Result<(), ER> {
        Ok(())
    }

    #[test]
    #[should_panic]
    fn returning_error() -> Result<(), ER> {
        Err(ER{})
    }
}
