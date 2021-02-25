fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // E0384: cannot assign twice to immutable variable `x`
    let x = 6; // shadowing 隐藏了上面的 let x
    println!("The value of x is: {}", x);
}
