use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); 
    print!("y is {}", y);
    let y = "y * 100.0";
    println!("y result is {}", y);
    // print!("y result is {}", y);
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    if rand::random() {
        // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }
}
