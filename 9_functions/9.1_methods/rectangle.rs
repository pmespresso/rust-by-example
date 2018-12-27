struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // static methods are generally used as constructors
    // don't need to be called by an instance
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // another static method
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point
}

impl Rectangle {
    // This is an instance method
    // &self is syntactic sugar for self: &Self
    // where Self is the type of the caller object.

    // In this case, Self = Rectangle
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x2 - x1) * (y2 - y1)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x2 - x1).abs() + (y2 - y1).abs())
    }

    // this method requires the caller to be mutable
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // this method consumes the resources of the caller object
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // first and second go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        // static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0)
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(2.0, 2.0)
    };

    // rectangle.translate(1.0, 1.0); <--- Error! rectangle is immutable, but this method requires a mutable object

    square.translate(1.0, 1.0); // square is mutable so it can call translate. the self in translate functions signature refers to square.

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy(); // first call to destroy consumes pair, i.e. the function signature takes ownership of `self` rather than referencing it.

    // pair.destory(); // so the second time calling it, pair is out of scope and compiler will complain.
}
