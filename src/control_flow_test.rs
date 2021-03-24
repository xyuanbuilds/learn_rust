fn if_else_test(num: u8) -> u8 {
    let n = if num < 255 { num } else { 0 };
    n
}

// 自行 break 控制循环停止的方式
fn loop_test(mut num: u8) -> u8 {
    loop {
        if num == 2 {
            continue;
        } else {
            println!("{}", num)
        }
        num += 1;
        if num == 10 {
            break num;
        }
    }
}

// 设置条件控制循环停止的方式

fn while_test(mut num: u8) -> u8 {
    while num < 10 {
        num += 1;
        if num == 2 {
            continue;
        }
        println!("{}", num);
    }
    num
}

#[test]
fn test() {
    let a = while_test(0);
}

fn for_range_test(num: u8) {
    // 范围右侧等号来包含结尾条件
    for n in num..=10 {
        println!("{}", n)
    }
}
#[test]
fn test1() {
    for_range_test(0);
}

/// enum的每一项需要定义 名称/标签 及值类型 标签(值类型)
/// 也可以 单标签，不含值，这时候就相当于只是一个判断 标签
#[derive(Debug)]
enum ReturnType {
    Pair((i32, i32)),
    Some(i32),
    None,
}

/// 通过 use 可以将 enum 中的项在当前 mod 中生成一个别名
use ReturnType::Pair;

fn test_data(param: Option<u8>) -> ReturnType {
    if param == Some(1) {
        Pair((2, -2))
    } else if param == Some(2) {
        ReturnType::Pair((2, 3))
    } else if param == Some(0) {
        ReturnType::Pair((1, -1))
    } else if param == Some(3) {
        ReturnType::Some(1)
    } else {
        ReturnType::None
    }
}

/// # 模式匹配
/// 除了匹配额外还提供了：Destructuring、Guards、Binding；
///
/// Destructuring: 解构，相当于分解某个复合类型，提取某个位置的特定值进行匹配；
///
/// Guards: 一般用于结构后，相当于提取后能再进行表达式计算对返回值验证，多过滤一次；
///
/// Binding: @ 相当于一个占位效果，占位提取出当前匹配的值，并绑定给左侧变量名，间接访问变量，使得当前分支能获取值；
fn match_test(num: u8) {
    let res = test_data(Some(num));
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", res);
    match res {
        // Destruct the first and match 1
        ReturnType::Pair((1, ..)) => println!("First is `1` and the rest doesn't matter"),
        // The ^ `if condition` part is a guard
        ReturnType::Pair((x, y)) if x + y == 0 => println!("Antimatter, kaboom!"),
        ReturnType::Pair((x, _)) if x % 2 == 1 => println!("The first one is odd"),
        // 提取 some 中的数值并匹配是否为 1，再绑定到 n 上以供分支使用
        ReturnType::Some(n @ 1) => println!("get {}", n),
        // 模式匹配是需要穷尽的，如果需要统一处理剩余，则使用通配符 _
        _ => println!("No correlation..."),
    }
}

#[test]
fn test_enum_match() {
    for n in 0..=4 {
        match_test(n)
    }
}

/// if let 相当于将 bool 判断条件，变为 模式匹配
/// if let 相当于一个 只处理一种模式的 match
/// if let 模式 = 值/变量 { 匹配后操作 }
fn if_left_test(num: u8) {
    if let ReturnType::Pair((1, ..)) = test_data(Some(num)) {
        println!("First is `1` and the rest doesn't matter")
    }
}

fn if_left_test1(num: u8) {
    if let ReturnType::Some(n @ 10) = test_data(Some(num)) {
        println!("get {}", n)
    }
}

#[test]
fn test_if_left() {
    if_left_test(0)
}

fn while_left_test() {
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
}
