use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("all element {:?}", slice);
    println!("first element {}", slice[0]);
    println!("slice len {}", slice.len());
}

fn array_and_slice() {
    let xs: [i32; 5] = [1, 3, 5, 6, 3];
    let ys: [i32; 10] = [100; 10];
    println!("{:?}, {}, {}\n {}", xs, xs[0], xs[4], xs.len());
    println!("{:?}, {}", ys, ys.len());

    println!("use memory {} size", mem::size_of_val(&xs));

    analyze_slice(&xs);

    analyze_slice(&xs[0 .. 2]);

    println!("前");
    analyze_slice(&xs[3 .. ]);

    println!("后");
    analyze_slice(&xs[ .. 4]);
}