fn fn_closures_capture() {
    use std::mem;

    let color = "green";
    let print = || println!("hehe {}", color);
    print();
    print();

    let mut count = 1;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();

    // let reborrow = &mut count;

    let movable = Box::new(3);
    let consume = || {
        println!("movable: {}", movable);
        mem::drop(movable);
    };

    consume();
//    consume();
}