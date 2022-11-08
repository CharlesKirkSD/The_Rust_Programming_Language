fn main() {
    let s1 = String::from("Hello");

    let s1_length = calculate_length(&s1);
    println!("The length of {} is {}", s1, s1_length);

    let mut s2 = String::from("Hello");
    change_string(&mut s2);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" World!");
}