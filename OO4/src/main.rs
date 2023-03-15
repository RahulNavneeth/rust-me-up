use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let value : i32 = rand::thread_rng().gen_range(0..=100);
    let mut stop : bool = false;
    println!("START GUESSING");
    while !stop {
        let mut input : String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(i) => {
                match i.cmp(&value) {
                    Ordering::Less => println!("VALUE IS LOW"),
                    Ordering::Greater => println!("VALUE IS HIGH"),
                    Ordering::Equal => { 
                        println!("GUESSED THE CORRECT VALUE");
                        stop = true;
                    },
                }
            },
            Err(_) => {
                println!("INVALID INPUT");
            },
        }
    }
    println!("\n ^-^")
}

