pub mod control_flow_test;
pub mod hashmap_test;
mod reference_test;
mod sourcemap_test;
pub mod struct_test;
pub mod vector_test;

use std::io;
fn main() -> io::Result<()> {
    println!("Hello, world!");
    let x = 5;
    const X1: i32 = 5; // 常量必须定义类型，且尽量全大写
    println!("The value of x is: {} & x1: {}", x, X1);
    // x = 6; // E0384: cannot assign twice to immutable variable `x`
    let x = 6; // shadowing 隐藏了上面的 let x
               // const X1: i32 = 5; // 常量不可 shadowing
    println!("The value of x is: {}", x);

    let u16test: u16 = 222;
    // let u8test: u8 = 256; // u8 最大 255

    let char_test: char = 'ℤ'; // 单引号容量小
                               // let nextTest = '中文'; // ERROR: Literal must be one character long
    let string_test = "中文";
    println!("{} {} {}", u16test, char_test, string_test);

    let tup_test = (500, "中文", 1); // 定长，长度不可变
    let (_, __, tup3) = tup_test;
    println!("{} {} {}", tup_test.0, tup_test.1, tup3);

    let array_test: [i32; 2] = [1, 2]; // 长度为2 的 i32数组
    let array_test1 = [2; 3]; // 长度为3，3个元素都为2
    let vector_test: Vec<i32> = vec![1, 2, 3, 45]; // 长度可变使用 vector
    println!("{} {} {}", array_test[1], array_test1[1], vector_test[1]);

    // 1 1 2 3 5 8 13 ...
    fn fibonacci(n: i32) {
        let mut x = 0;
        let mut y = 1;
        let mut z = x + y;
        // println!("begin: {} {}", x, 0);
        // range 用法，常用于数值遍历及遍历中取数
        for num in 1..n {
            x = y;
            y = z;
            z = x + y;
            println!("{} {}", x, num);
        }
    }
    fibonacci(10);

    use sourcemap::SourceMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let p = Path::new("src/test.map");

    println!("path: {:?}", p);
    let mut f = File::open(p)?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    println!("token: {:#?}", buffer);
    Ok(())
}

#[cfg(test)]
mod tests_main {
    use crate::struct_test;
    // mod 为 tests_main use 不会导入 struct_test 文件中的测试 mod test
    // 但如果 main 文件中的测试 mod 也叫 test，则会同时执行 struct_test 中的 test
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = struct_test::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = struct_test::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
