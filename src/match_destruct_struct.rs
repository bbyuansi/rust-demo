fn match_destruct_struct() {
    // 解构结构体
    struct Foo { x: (u32, u32), y: u32 };
    let foo = Foo { x: (1, 3), y: 4 };
    let Foo { x: (a, b), y } = foo;
    println!("{}, {}, {}", a, b, y);
    let Foo { y, .. } = foo;
    println!("{}", y);
    let Foo { x: (q, r), .. } = foo;
    println!("{}", y);
    let Foo { x: g, y: i } = foo;
    println!("{:#?}, {:#?}", g, i);
}