fn main() {
    the_string_type();
    double_free_error();
    clone_method();
    stack_only_data_copy();
}

fn the_string_type () {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}",s);
}

fn double_free_error () {
    let s1 = String::from("Hello");
    let s2 = String::from(", World!");
    let s3 = s1;
    // println!("{}{}", s1, s2); // Uncommenting this line results in a Compiler Error, "borrow of moved value `s1`", as s1 is no longer valid.
    println!("{}{}", s3, s2);
}

fn clone_method() {
    let s1 = String::from("Hello");
    let s2 = String::from(", World!");
    let s3 = s1.clone();
    println!("{}{}", s1, s2);
    println!("{}{}", s3, s2);
 }

 fn stack_only_data_copy() {
     let x :i32 = 990;
     let y = x;
     println!("x is {}, y is {}", x, y);
 }