#[cfg(test)]
mod tests {
    use std::any::type_name;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct V2 {
        odd: bool,
        nb: i16,
    }

    impl V2 {
        fn is_odd(&self) -> bool {
            self.odd
        }
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn unwrap_result() {
        use std::num::ParseIntError;
        fn do_unwrap_result() -> Result<i32, ParseIntError> {
            let v1 = "1".parse::<i32>()?;
            Ok(v1)
        }
        assert_eq!(do_unwrap_result(), Ok(1));
    }

    #[test]
    fn unwrap_option() {
        fn do_unwrap_option() -> Option<i32> {
            let o1 = Some(1)?;
            Some(o1)
        }
        assert_eq!(do_unwrap_option(), Some(1));
    }

    #[test]
    fn hashmap() {
        let mut hm = HashMap::new();
        hm.insert(String::from("k1"), 1);
        hm.insert(String::from("k2"), 2);
        assert_eq!(hm.get(&String::from("k1")), Some(&1));
        assert_eq!(hm.get(&String::from("k2")), Some(&2));
        // TODO https://doc.rust-lang.org/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value
    }

    #[test]
    fn borrow() {
        fn borrow(mystr: &String) -> &String {
            mystr
        }
        assert_eq!("hi borrow", borrow(&"hi borrow".to_string()));

        fn borrow_mutable(mystr: &mut String) -> &String {
            mystr.push_str("!!");
            mystr
        }
        assert_eq!(
            "hi mutable!!",
            borrow_mutable(&mut "hi mutable".to_string())
        );
    }

    #[test]
    fn format() {
        assert_eq!("Hello world!", format!("Hello world!"));
        assert_eq!("1-2", format!("{}-{}", 1, 2));
        assert_eq!("1-3", format!("{a}-{b}", a = 1, b = 3));
    }

    #[test]
    fn vector() {
        let v = vec![0, 2, 4];
        for vv in v {
            assert!(vv % 2 == 0);
        }
    }

    fn dbgformat_v2(v: &V2) -> String {
        format!("{:?}", v)
    }

    fn sort<F>(a: u16, b: u16, sort_asc: F) -> (u16, u16)
    where
        F: Fn(u16, u16) -> bool,
    {
        if sort_asc(a, b) {
            (a, b)
        } else {
            (b, a)
        }
    }
    #[test]
    fn closure_invoke() {
        let identity = |x| x;
        let one = || 1;
        assert_eq!(identity(5), 5);
        assert_eq!(one(), 1);

        fn gt1(a: u16, b: u16) -> bool {
            a <= b
        }
        let gt_closure = |a, b| a <= b;
        let gt_closure_typed = |a: u16, b: u16| -> bool { a <= b };
        assert_eq!(sort(1, 2, gt1), (1, 2));
        assert_eq!(sort(1, 2, gt_closure), (1, 2));
        assert_eq!(sort(1, 2, gt_closure_typed), (1, 2));
        //
        assert_eq!(sort(3, 1, gt1), (1, 3));
        assert_eq!(sort(3, 1, gt_closure), (1, 3));
        assert_eq!(sort(3, 1, gt_closure_typed), (1, 3));
    }
    #[test]
    fn call_parameterized_func() {
        let name_1 = type_name::<u8>();
        let name_2 = type_name::<f32>();
        assert_eq!(name_1, "u8");
        assert_eq!(name_2, "f32");
    }

    #[test]
    fn test_clone() {
        let n = V2 { odd: true, nb: 51 };
        assert_eq!("V2 { odd: true, nb: 51 }", dbgformat_v2(&n));

        let mut m = n.clone();
        m.nb += 100;
        assert_eq!("V2 { odd: true, nb: 51 }", dbgformat_v2(&n));
        assert_eq!("V2 { odd: true, nb: 151 }", dbgformat_v2(&m));
    }

    #[test]
    fn test_trait() {
        trait Signed {
            fn is_positive(&self) -> bool;
        }
        impl Signed for V2 {
            fn is_positive(&self) -> bool {
                self.nb > 0
            }
        }

        let s = V2 { odd: false, nb: 2 };
        assert!(s.is_positive());
    }
    #[test]
    fn test_match() {
        let n = 2;
        let res: &str = match n {
            1 => "c1",
            2 | 3 => "c2/3",
            _ => "other",
        };
        assert_eq!("c2/3", res);
    }

    #[test]
    fn if_let() {
        let s = V2 { odd: true, nb: 1 };
        if let V2 { odd: true, nb: _ } = s {
            assert_eq!(
                "V2 { odd: true, nb: 1 } and nb=1",
                format!("{:?} and nb=1", s)
            );
        } else {
            panic!("");
        }
        assert!(s.is_odd());
    }

    #[test]
    fn test_struct() {
        let x = {
            let a = 1;
            let b = 2;
            a + b
        };
        assert_eq!(3, x);
    }
}
