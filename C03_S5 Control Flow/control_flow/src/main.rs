fn main() {
    println!("Hello World!");

    let x = 12;
    let y = 3;
    let result_1 = power(12.0, 2);
    let result_2 = power_up(12.0,3);

    let operation;

    if y == 2 {
        operation = "squared"
    } else {
        operation = "cubed"
    }

    println!("{} squared is {}.", x, result_1);
    println!("{} {} is {}.", x, operation, result_2);

    println!("The function loopy returns {}", loopy());
    println!("The function while_demo return {}", while_demo());

    lift_off(10);
}

fn power(x: f64, y: i16) -> f64 {
    let result: f64;

    if y == 2 {
        result = x * x
    }
    else if y == 3 {
        result = x * x * x
    }
    else {
        result = x
    }
    result
}

fn power_up(x: f64, y: i16) -> f64 {
    let result: f64;

    result = if y == 2 {
        x * x
    }
    else if y == 3 {
        x * x * x
    }
    else {
        x
    };

    result
}

fn loopy() ->f32 {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10
        }
    };

    result as f32
}

fn while_demo() ->i32 {
    let mut index = 1;
     while index < 10 {
         index += 1;
     }
     index
}

fn lift_off(t_minus :i32) {
    for t_minus in (1..t_minus + 1).rev() {
        println!("{}", t_minus);
    }
    println!("Lift Off");
}