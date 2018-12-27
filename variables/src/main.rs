fn main() {
    // mutation
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 4;
    let y = 3;
    let y = 999;

    // y = 9; this will compile time error, and that's a key diff between shadowing and mut
    println!("value of y is: {}", y);

    // another thing is, shadowing allows us to change the variable's type, whereas
    // mut will not

    let spaces = "    ";
    let spaces = spaces.len();

    // vs.

    // let mut spaces = "    ";
    // spaces = spaces.len();
    //          expected type `&str`
    //          found type `usize`

    println!("spaces length is: {}", spaces);
}
