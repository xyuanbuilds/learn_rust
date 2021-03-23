use std::ops::Add;
fn n1(mut param1: String) -> String {
    println!("{}", param1);
    let inner = "sdf".to_string();
    let param1 = inner.clone();
    println!("{}", inner);
    let a = String::from("hello");
    let b = String::from(" world");
    let s = a.clone().add(&b);
    println!("{}, {}", a, s);
    println!("{}", b);

    let x: i32 = 1;
    let mut c = [1, 2, 3];
    let b = &c;
    let f = &c;
    let d = &b;
    let e = &mut c;
    e[0] = 5;
    param1
}

#[test]
fn test1() {
    let n1_test = String::from("dfdfd");
    let a = n1(n1_test);
    println!("{}", a)
}

fn n2(param1: &mut String) {}
