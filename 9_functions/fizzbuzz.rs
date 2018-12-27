/*
"Write a program that prints the numbers from 1 to 100.
But for multiples of three print “Fizz” instead of the number and
for the multiples of five print “Buzz”.
For numbers which are multiples of both three and five print “FizzBuzz”."

*/

// Unlike C/C++ there is no restriction to to order of function definitions
fn main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false
    }

    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("{}", "FizzBuzz")
    } else if is_divisible_by(n, 5) {
        println!("{}", "Buzz")
    } else if is_divisible_by(n, 3) {
        println!("{}", "Fizz")
    } else {
        println!("{}", n)
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for i in 1..=n+1 {
        fizzbuzz(i)
    }
}
