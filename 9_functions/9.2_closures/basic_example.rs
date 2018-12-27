fn main() {
    // Increment via closures and functions.
    fn function (i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless Functions
    // are assigned to appropriately named variables
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i | i+ 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // a closure taking no arguments which returns an inferred i32
    let one = || 1;
    println!("clsoure returning one: {}", one());
}
