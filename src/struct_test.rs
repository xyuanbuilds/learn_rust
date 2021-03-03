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
