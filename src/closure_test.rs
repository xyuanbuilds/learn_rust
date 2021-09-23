use std::str;

struct Home {
    size: &'static str,
    name: String,
}

impl Home {}

fn main() {
    let st_test = "Pascal".to_string();
    let home = Home {
        size: "test",
        name: st_test,
    };
    let get_home = || home;
    get_home();
}
