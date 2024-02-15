use rand::{thread_rng, Rng};
use rust_library::version_match;

fn main() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen();

    println!("A random value: {}", x);

    let m = version_match("1.0.0", ">= 1.0.0");
    match m {
        true => println!("1.0.0 >= 1.0.0: true"),
        false => println!("1.0.0 >= 1.0.0: false"),
    }

    let m = version_match("0.3.0", ">= 1.0.0");
    match m {
        true => println!("0.3.0 >= 1.0.0: true"),
        false => println!("0.3.0 >= 1.0.0: false"),
    }
}
