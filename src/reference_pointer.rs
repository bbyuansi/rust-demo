fn reference_pointer() {
    let reference = &4;
    match reference {
        &val => println!("{:?}", val)
    }

    match *reference {
        val => println!("{}", val)
    }

    let ref is_a_reference = 3;
    println!("{}", *is_a_reference);

    let value = 5;
    let mut ref_value = 6;

    match value {
        ref r => println!("{:#?}", r)
    };

    match ref_value {
        ref mut m => {
            *m += 10;
            println!("{:#?}", m);
        }
    }
}