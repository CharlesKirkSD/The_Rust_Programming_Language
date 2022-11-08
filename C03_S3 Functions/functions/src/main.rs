fn main() {
    println!("Hello World!");
    another_function(12);

    let cube_result = cube(2,3,4);
    println!("cube(2,3,4) produces the result {}", cube_result)
}

fn another_function(x: i32) {
    println!("The square of {} is {}", x, x * x);
}

fn cube(x: i32, y: i32, z: i32) -> i32 {
    return x * y * z
}