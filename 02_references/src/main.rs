fn calculate_lenght(s : &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("Hello");

    let len = calculate_lenght(&s1);

    println!("The lenght of '{s1}' is {len}.");
}
