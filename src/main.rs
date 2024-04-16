
const SCREAMING_SNAKE_CASE: f64 = 0.2;
fn main() {
    let mut bunnies: i32 = 2;
    println!("initial bunnies, world! {}", bunnies);
    let (x, y) = (8, 50);

    {
        bunnies = 5;
        let _x = 0;
        println!("{}", bunnies + _x)
    }
    println!("\nHello, world! {}, {}", bunnies, SCREAMING_SNAKE_CASE);
    println!("\nHello2, world! {}", y * x);

    let news = 8;
    let ready = 2;
    println!("Firing {} of my {} news...", ready, news);
    let res = do_stuff(1.0, 2.0);
    println!("{}", res)
}

//lowercase separated by _
fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}", qty, oz);
    qty * oz
}
