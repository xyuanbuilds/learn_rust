use std::collections::HashMap;

#[test]
fn basic() {
    let mut scores = HashMap::new();

    let a = "Blue".to_string();
    let b = "Red";

    // k 需要是一个 String，&str 不行
    scores.insert(a.clone(), 10);
    scores.insert(b.to_string(), 20);
    // scores.insert(String::from("Yellow"), 50);

    #[derive(Hash, Debug, Clone, Eq)]
    struct T {
        a: i32,
    }
    /// 用于比较两个 T 是否相等。因为 HashMap::get 需要比较 k 是否相等
    /// 所以这个 trait 的实现是必要的
    ///
    /// 当然也可以使用 #[derive(PartialEq)] 来快速实现
    /// 自动实现的会比较 struct 中所有 k -> v 值
    impl PartialEq for T {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a
        }
    }
    let s_test = T { a: 1 };

    let mut s_map = HashMap::new();
    s_map.insert(s_test.clone(), 10);

    // get 需要从一个 &x 取值
    if let Some(n @ 10) = scores.get::<str>(&a) {
        println!("{}", n)
    }
    if let Some(n @ 20) = scores.get::<str>(b) {
        println!("{}", n)
    }
    if let Some(n @ 10) = s_map.get(&s_test) {
        println!("{}", n)
    }
}

/// HashMap 会拥有插入的堆中数据的所有权
/// 也就是 插入HashMap 是一个移动操作
fn ownership_test() {
    let mut scores = HashMap::new();

    let a = String::from("lalalaland");

    scores.insert('0', a);
    // println!("{}", a); // E0382
}
