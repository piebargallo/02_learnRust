fn main() {
    let rect1 = (30, 50);
 
    println!(
         "The area of the rectangle is {} aquare pixels.",
         area(rect1)
    );
 }
 
 fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
 }
 