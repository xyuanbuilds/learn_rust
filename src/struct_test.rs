// struct 定义属性
#[derive(Debug)] // 让该struct可以使用 {:?} 和 {:#?}
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// impl 定义方法
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
// 相当于一个构造函数
// 也可定义在 impl 结构内，用 new
pub fn r_c(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn one() {
        assert_eq!(2 + 2, 4);
    }

    // fail test
    #[test]
    fn larger_can_hold_smaller() {
        let a = r_c(1, 2);
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// impl Point<u16> {
//     // new 关联函数相当于 构造函数
//     fn new(x: u16, y: u16) -> Self {
//         Point { x, y }
//     }

//     // 方法
//     // 可变 self 也必须
//     fn change(&mut self, x: u16, y: u16) {
//         self.x = x;
//         self.y = y;
//     }

//     // 关联函数
//     fn area(&self) -> u16 {
//         self.x * self.y
//     }
// }

// #[test]
// fn test_point_u16() {
//     let mut a = Point::new(100, 100);
//     assert!(a.area() == 10000);
//     a.change(2, 2);
//     assert!(a.area() == 4);
// }

trait Add<R = Self> {
    type Out; // 关联类型
    fn add(self, rhs: R) -> Self::Out;
}

// 一个文件中一个 struct 只能 impl 一次
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// type T = u32 + i16;
// impl Add for Point<T> {
//     type Out = T;
//     fn add(self, rhs: Self) -> Self::Out {
//         self.x + rhs.x
//     }
// }
// impl Add for Point<u32> {
//     type Out = u32;
//     fn add(self, rhs: Self) -> Self::Out {
//         self.x + rhs.x
//     }
// }
