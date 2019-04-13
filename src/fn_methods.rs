#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 10_f32, y: 20_f32 }
    }

    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Reactangle {
    p1: Point,
    p2: Point,
}

impl Reactangle {
    fn area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x2 - x1) * (y2 - y1)).abs()
    }

    fn perimeter(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x2 - x1).abs() + (y2 - y1).abs()) * 2_f32
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("destroy Pair {}, {}", first, second);
    }
}


fn fn_methods() {
    let reactangle = Reactangle {
        p1: Point::origin(),
        p2: Point::new(40_f32, 50_f32),
    };

    println!("area: {}, perimeter: {}", reactangle.area(), reactangle.perimeter());

    let mut square = Reactangle {
        p1: Point::origin(),
        p2: Point::new(11_f32, 22_f32),
    };

    square.translate(2_f32, 3_f32);
    println!("{:#?}", square);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}