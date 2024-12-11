fn main() {
    let mut s = String::from("Hello");

    // push_str() appends a literal to a String
    s.push_str(", World");
    
    println!("{s}");
}
