fn main() {
    // move example
    // let s1 = String::from("Hello");
    // // let s2 = s1; // this will not work as String does not implement a Copy trait like integers
    // let s2 = s1.clone(); // to make deep copy of data in the heap, we use the clone method implemented by the String trait
    // // Also note that one can't implement copy and clone at the same time
    // println!("The original string is {}", s1);

    // Stack only data: Copy
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);
    // this works because integers are stored in the stack and has implemented the copy trait

    // Ownership and functions
    // let s = String::from("hello");
    // take_ownership(s);
    // // println!("s = {}", s); // this won't work as take_ownership has the ownership of s now
    // let x = 5;
    // makes_copy(x);
    // println!("x = {}", x); // this would work as x is stored in the stack and has the copy trait

    // Return values and scope
    //     let s1 = gives_ownership();
    //     let s2 = String::from("hello");
    //     let s3 = takes_and_gives_back(s2);

    // References and borrowing
    // let s1 = String::from("hello"); this wont work with mutable reference with the mut keyword as complier need to know this value change change either via a reference or by direct assignment
    // let mut s1 = String::from("hello");
    // let len = calculate_length(&s1); // this passes a reference(basically a pointer) to string instead of giving the ownership of the string to the function

    // Mutable reference
    // change(&s1); // this wont work as references are immutable by default, Like all other variables we will need to mark this with the mut keyword so that we tell the compiler to expect changes in this reference
    // change(&mut s1);
    // println!("Length of the string {} is {}", s1, len);

    // mutable reference limitation
    // following wont work as you can't create two mutable reference of a variable in a single scope. This is done so that data race conditions won't occur by simultaneous mutation of the variable s.`
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // r1.push_str("test");
    // r2.push_str("test1");
    // println!("{} , {}", r1, r2);

    // Dangling references
    // the next line will won't work as the compiler guarantees that there will be no dangling reference after the s variable is dropped out of the scope
    // let reference_to_nothing = dangle();
}

// fn take_ownership(s: String) {
//     println!("{}", s);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> String {
//     let s = String::from("hellow");

//     s
//     // S is dropped so &s will contain reference to nothing
// }