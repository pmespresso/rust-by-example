fn main() {
    use std::mem;

    let color = "green";

    // A closure to print 'color' which immediately borrows ('&')
    // 'color' and stores the borrow and closure in the 'print'
    // variable. It will remain borrowed until 'print' goes out of
    // scope. 'println!' only requires 'by reference' so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    print(); // call the closure using borrow
    print();

    let mut count = 0; // copy type, i.e. stack based memory

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // let _reborrow = &mut count; // this won't work as only one mutable refrence per scope is allowed.

    let movable = Box::new(3); // a non-copy type, i.e. heap based memory

    // N.B. all vaues in Rust are stack allocated by default. Values can be `Boxed` (allocated in the heap) by creating a Box<T>.
    // A box is a smart pointer to aheap allocated value of type T.

    let consume = || {
        println!("`movable`: {:?}", movable);  // movable is owned in this scope now
        mem::drop(movable);
    };

    consume(); // movable ownership moves to to consume scope, leaves this scope.

    // consume(); // so this won't work

    let haystack = vec![1, 2, 3]; // `Vec` has non-copy semantics

    // using `move` keyword before || forces closure to take ownership of captured variables
    let contains = move |needle| haystack.contains(needle); // haystack ownership is forced into this scope

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There are {} elements in vec", haystack.len()); // this errors saying `valued moved (into closure)` becusase haystack has type std::vec::Vec<i32>, which does not implement the Copy trait`
    // but if you remove the `move` keyword, this works because haystack only gets borrowed in the closure scope
}
