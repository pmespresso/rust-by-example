// struct UnPrintable(i32);
use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// In order to use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax (i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}


fn main() {
  // println!("Hello World");
  // println!("{a}{b}{c} things to do", a="mister", b="man", c="do");

  // println!("This is not a printable structure {:?}", UnPrintable(3));
  // println!("This is a printable structure {:?}", Structure(3));

  // let name = "YJ";
  // let age = 22;
  // let yj = Person { name, age };
  //
  // println!("This is a readble Person struct {:?}", yj);

  let minmax = MinMax(1, 999);

  println!("Compare displays");
  println!("Display: {} ", minmax);
  println!("Debug: {:?} ", minmax);

  let point2d = Point2D { x: 3.3, y: 9.9 };

  println!("Display: {}", point2d);
  println!("Debug: {:?}", point2d);

  let v = List(vec![1, 2, 3]);
  println!("Vector -> {}", v);
}
