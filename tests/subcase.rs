use subcase::with_subcases;

with_subcases! {
    #[test]
    fn test_one() {
        let mut v = vec![1,2,3];

        subcase! {{
            v.push(4);
        }}
        subcase! {{
            v.push(5);
        }}

        assert_eq!(v.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_two() {
        let mut v = vec![1,2,3];

        subcase! {{
            v.push(4);
        }}
        subcase! {{
            v.push(5);
        }}

        assert_eq!(v.len(), 5);
    }
}
