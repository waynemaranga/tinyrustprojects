#[allow(dead_code)]
mod fizzbuzz;
mod stem_and_leaf;
fn main() {
    println!("Hello, Stem and Leaf Plot!");
    stem_and_leaf::driver();
    fizzbuzz::driver();
}
