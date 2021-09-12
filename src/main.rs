use std::any::type_name;

#[derive(Debug, Clone)]
struct V2 {
    odd: bool,
    nb: i16,
}

impl V2 {
    fn is_odd(self) -> bool {
        self.odd
    }
}

fn main() {
    test_format();
    test_scope();
    test_if_let();
    test_match();
    test_trait();
    test_clone();
    test_call_parameterized_func();
    test_vec();
    test_lambda_invoke();
    test_borrow();
    Some(1);
}

fn test_borrow() {
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

fn test_format() {
    assert_eq!("Hello world!", format!("Hello world!"));
    assert_eq!("1-2", format!("{}-{}", 1, 2));
    assert_eq!("1-3", format!("{a}-{b}", a = 1, b = 3));
}

fn test_vec() {
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
    return if sort_asc(a, b) { (a, b) } else { (b, a) };
}

fn test_lambda_invoke() {
    assert_eq!((|x| x)(5), 5);
    assert_eq!((|| 1)(), 1);

    fn gt1(a: u16, b: u16) -> bool {
        return if a <= b { true } else { false };
    }
    let gt_lambda = |a, b| a <= b;
    let gt_lambda_typed = |a: u16, b: u16| -> bool { a <= b };
    assert_eq!(sort(1, 2, gt1), (1, 2));
    assert_eq!(sort(1, 2, gt_lambda), (1, 2));
    assert_eq!(sort(1, 2, gt_lambda_typed), (1, 2));
    //
    assert_eq!(sort(3, 1, gt1), (1, 3));
    assert_eq!(sort(3, 1, gt_lambda), (1, 3));
    assert_eq!(sort(3, 1, gt_lambda_typed), (1, 3));
}

fn test_call_parameterized_func() {
    let name_1 = type_name::<u8>();
    let name_2 = type_name::<f32>();
    assert_eq!(name_1, "u8");
    assert_eq!(name_2, "f32");
}

fn test_clone() {
    let n = V2 { odd: true, nb: 51 };
    assert_eq!("V2 { odd: true, nb: 51 }", dbgformat_v2(&n));

    let mut m = n.clone();
    m.nb += 100;
    assert_eq!("V2 { odd: true, nb: 51 }", dbgformat_v2(&n));
    assert_eq!("V2 { odd: true, nb: 151 }", dbgformat_v2(&m));
}

fn test_trait() {
    trait Signed {
        fn is_positive(self) -> bool;
    }
    impl Signed for V2 {
        fn is_positive(self) -> bool {
            return self.nb > 0;
        }
    }

    let s = V2 { odd: false, nb: 2 };
    assert_eq!(true, s.is_positive());
}

fn test_match() {
    let n = 2;
    let res: &str = match n {
        1 => "c1",
        2 | 3 => "c2/3",
        _ => "other",
    };
    assert_eq!("c2/3", res);
}

#[allow(unreachable_code)]
fn test_if_let() {
    let s = V2 { odd: true, nb: 1 };
    if let V2 { odd: true, nb: _ } = s {
        assert_eq!(
            "V2 { odd: true, nb: 1 } and nb=1",
            format!("{:?} and nb=1", s)
        );
    } else {
        panic!("");
    }
    assert_eq!(true, s.is_odd());
}

#[allow(unreachable_code)]
fn test_scope() {
    let x = {
        let a = 1;
        let b = 2;
        a + b
    };
    assert_eq!(3, x);
}
