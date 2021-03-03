use std::io::prelude::*;

#[test]
fn read_test() -> std::io::Result<()> {
    use std::fs::File;

    let mut f = File::open("src/test.text")?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    let s = String::from_utf8_lossy(&buffer); // 读取为字符串
                                              // let b = s.as_bytes();
    println!("token: {:?}", s);

    Ok(())
}

#[test]
fn basic() -> std::io::Result<()> {
    use sourcemap::{SourceMap, SourceView};
    let input = include_bytes!("layouts.map"); // 读取整个 map 文件
    let f = include_str!("layouts.js");
    // let sm = SourceMap::from_reader(&input[..]).unwrap();
    let sm = SourceMap::from_reader(&input[..]).unwrap(); // 生成用于使用的 sourceMap 实例
    let sv = SourceView::new(f); // 读取打包后源文件

    // for t in sm.tokens() {
    //     println!("{:#?}", t.to_tuple());
    // }
    let get_o = sm.lookup_token(0, 47055);
    println!("{:#?}", get_o);
    Ok(())
}
