// 相干性（coherence）/孤儿规则（orphan rule）
// 只有当 trait 本身或 要实现 trait 当类型 位于 crate 的本地作用域时，次啊能为该类型实现 trait
// crate 相当于一个根节点。
pub trait Summary {
    // trait 可以定义方法类型，同时也可以提供默认实现
    fn summarize(&self) -> String {
        String::from("default impl")
    }
}

#[derive(Debug)]
struct Tweet {
    pub username: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("my name: {}", self.username)
    }
}

#[test]
fn basic_test() {
    let a = Tweet {
        username: String::from("john"),
    };

    println!("{}", a.summarize())
}

// trait 可以作为参数，代表该入参需要是一个实现了该trait的类型
// impl Summary：某种 实现了 Summary trait 的类型
pub fn notify(cur: impl Summary) {
    println!("{}", cur.summarize())
}

// trait bound 是基本语法，表示 T类型实现了Summary，上面的写法属于语法糖，且失去了 T占位
// 也就是实现了 Summary trait 的 T 类型
pub fn notify_generic<T: Summary>(cur: T) {
    println!("{}", cur.summarize())
}

#[test]
fn trait_bound_test() {
    let a1 = Tweet {
        username: String::from("john"),
    };

    notify(a1);

    let a2 = Tweet {
        username: String::from("john"),
    };
    notify_generic(a2);
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 任意 T 类型都实现 new
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只为拥有 Display&PartialOrd的T类型 实现 cmp_display 方法
// 对任何满足特定 trait bound 的类型实现 trait 被称为
// blanket implementations
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// trait 和 trait bound 让我们使用泛型类型参数来减少重复，
// 并仍然能够向编译器明确指定泛型类型需要拥有哪些行为。
// 因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用到的具体类型是否提供了正确的行为。
