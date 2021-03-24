fn basic() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn borrow_with_push() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // E0502: cannot borrow `v` as mutable
    // because it is also borrowed as immutable mutable borrow occurs here
    v.push(6); // ! 借用的生命周期还未结束，不可改变 vec
               // 不能这么做的原因是由于 vector 的工作方式：
               // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，
               // 可能会要求分配新内存并将老的元素拷贝到新的空间中。
               // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

    // 不使用 first 它的生命周期也就不会持续到这里
    // println!("The first element is: {}", first);
}

#[test]
fn vec_iterator_test() {
    let mut v = vec![1, 2, 3];

    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }
}

/// vec 理论上只能存放一种类型，那如果需要存放多种
/// 需要使用枚举来列出所有的类型，再用枚举来使用类型
/// 然后再用模式匹配来穷尽可能，来使用真实值
#[test]
fn vec_enum_test() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Int(n) => println!("{:?}", n),
            SpreadsheetCell::Text(s) => println!("{:?}", s),
            SpreadsheetCell::Float(f) => println!("{:?}", f),
        }
    }
}
