#[test]
pub fn eg() {
    // 栈中存不可变数据，也就是容量不变的数据，堆中存容量可变数据
    let s1 = "hello"; // 栈中
    let mut s2 = String::from("hello"); // { ptr -> 堆; len; capacity}
    s2 = "dfdfd".to_string(); // "xxx" 为 &str 不可变，要转为可变需要.to_string();
    return ();
}

#[test]
fn shadow_test() {
    let x = 1; // 第一次定义会被第二次定义隐藏
    let x = 2;
    println!("{}", x);
}

#[test]
fn move_test() {
    let s1 = String::from("hello");
    let s2 = s1; // 这样的写法不会成功复制引用，只会让上一个引用失效
                 // println!("{}", s1); // E0382 引用失效，引用s1被移动到了s2上
    println!("{}", s2); // E0382 引用失效，引用s1被移动到了s2上
}

#[test]
// 堆中结构只能通过clone来进行变量间传递（上一个变量不失效）
// 一些简单标量值则可以实现变量传递不失效
fn clone_test() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    // copy
    let x = 5;
    let y = x;
    //那么什么类型是 Copy 的呢？可以查看给定类型的文档来确认，
    //不过作为一个通用的规则，任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的。如下是一些 Copy 的类型：
    //所有整数类型，比如 u32。
    //布尔类型，bool，它的值是 true 和 false。
    //所有浮点数类型，比如 f64。
    //字符类型，char。
    //元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

    println!("x = {}, y = {}", x, y);
}
