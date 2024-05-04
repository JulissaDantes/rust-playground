
use std::error::Error;

//use goose::prelude::*;
use playground::greet;
use playground::words;
const SCREAMING_SNAKE_CASE: f64 = 0.2;
//#[tokio::main]
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
    println!("{}", res);

   // playground::greet(); if not using use
   greet();
   let res = playground::loops(x, y);
   println!("RES {}", res);

   let a = 1;
   let b = a.clone();
   println!("{} {}", a, b);
   let square:[[char;3]; 4] = [['d','e','w'],['t','s','b'],['a','n','t'],['l','i','o']];
   let w = words::get_words(Some('t'), Some(&square.concat()), square);
   println!("{} {:?}", w.len(), &w[0..]);//{:?} for the contents
}

//lowercase separated by _
fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}", qty, oz);
    qty * oz
}