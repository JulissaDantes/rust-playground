
use std::error::Error;

//use goose::prelude::*;
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

   let res = playground::loops(x, y);
   println!("RES {}", res);

   let a = 1;
   let b = a.clone();
   println!("{} {}", a, b);
   let square:[[char;3]; 4] = [['i','k','r'],['p','e','w'],['t','o','l'],['m','n','c']];
   let mut w = words::get_words(Some('e'), Some(&square.concat()), square);
   sort_words(&mut w);
   print_grouped_words(&w);
  // println!("{} {:?}", w.len(), &w[0..]);//{:?} for the contents
}

//lowercase separated by _
fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}", qty, oz);
    qty * oz
}

fn sort_words(w: &mut Vec<String>) {
        // Sort by length and then alphabetically.
        w.sort_by(|a, b| {
            let len_cmp = a.len().cmp(&b.len()); // Compare by length first
            if len_cmp == std::cmp::Ordering::Equal {
                a.cmp(b) // If lengths are equal, compare alphabetically
            } else {
                len_cmp
            }
        });
}

fn print_grouped_words(words: &Vec<String>) {
    let mut current_length = 0;
    for word in words {
        if word.len() != current_length {
            current_length = word.len();
            println!("{} letters:", current_length); // Print length header
        }
        println!("  {}", word); // Print word indented under the header
    }
}