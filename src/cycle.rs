#![allow(unreachable_code)]

fn cycle() {
    let mut counter = 1;
    'outer: loop {
        'inner: loop {
            counter = counter + 1;
            if counter >= 5 {
                break 'outer;
            }
            if counter >= 3 {
                break;
            }
        }
    }
    println!("{}", counter);

    let mut nums = 0;
    let result = loop {
        nums += 1;
        if nums >= 10 {
            break nums;
        }
    };
    println!("{}", result);

    let mut n = 1;
    while n <= 100 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz")
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

