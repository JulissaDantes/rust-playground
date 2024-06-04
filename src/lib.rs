pub mod words;

pub fn greet() {
    println!("Hi there")
}

pub fn loops(x: i32, y:i32) -> i32 {
    'bob: loop {
        println!("Bob");
        loop {
            println!("Second");
            loop {
                println!("Last");
                break 'bob;
            }
        }
    }
    x + y
}
