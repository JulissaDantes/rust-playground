
const SCREAMING_SNAKE_CASE: f64 = 0.2;
fn main() {
    let mut bunnies: i32 = 2;
    let (x, y) = (8, 50);

    {
        bunnies = 5;
        let _x = 0;
        println!("{}", bunnies + _x)
    }
    println!("\nHello, world! {}, {}", bunnies, SCREAMING_SNAKE_CASE);
    println!("\nHello2, world! {}", y * x);
}
