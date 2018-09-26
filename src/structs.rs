#[derive(Debug)]
struct Nil;

#[derive(Debug)]
struct Pair(i32, i32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1: my_p1, p2: my_p2 } = rect;
    let Point { x: my_x, y: my_y } = my_p1;
    my_x * my_y / 2f32
}

fn square(p: Point, x1: f32) -> Rectangle {
    let p2 = Point { x: p.x + x1,  y: p.y + x1 };
    Rectangle { p1: p, p2}
}

fn structs() {
    let point: Point = Point { x: 100f32, y: 200f32 };
    println!("point is x: {}, y: {}", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;
    println!("{}, {}", my_x, my_y);

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };
    println!("{:#?}", _rectangle);

    let _nil = Nil;
    println!("{:#?}", _nil);

    let pair = Pair(1, 100);
    println!("{:#?}", pair);

    let Pair(integer, deicimal) = pair;
    println!("{}, {}", integer, deicimal);

    println!("rectangle is {}", rect_area(_rectangle));

    let p1 = Point { x: 100f32, y: 200f32 };
    println!("square is {:#?}", square(p1, 10f32));
}