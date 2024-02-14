use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen();

    println!("A random value: {}", x);
}
