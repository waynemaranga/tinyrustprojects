use colored::Colorize;

pub fn driver() {
    // Regular fizzbuzz
    println!("Round 1...");
    fizzbuzz_1();
    // fizzbuzz with match
    println!("\nRound 2, and coloured...");
    fizzbuzz_2();
    // fizzbuzz with (unnecessary traits)
    println!("\nRound 3...");
    let mut fb_vector: Vec<u8> = Vec::new();
    for j in 1u8..=100 {
        fb_vector.push(j)
    }
    /* using loop with an iterator
    (1u8..=100).for_each(|j| {
        fb_vector.push(j)
    });
    */
    fizzbuzz_3(&fb_vector);
}

fn fizzbuzz_1() {
    for i in 1u8..=100 {
        let div_3: bool = i % 3 == 0;
        let div_5: bool = i % 5 == 0;

        if (div_3, div_5) == (true, true) {
            println!("FizzBuzz!")
        } else if (div_3, div_5) == (true, false) {
            println!("Fizz")
        } else if (div_3, div_5) == (false, true) {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}
fn fizzbuzz_2() {
    // coloured strings
    let fizz: colored::ColoredString = String::from("Fizz").blue();
    let buzz: colored::ColoredString = String::from("Buzz").red();
    let fizz_buzz: colored::ColoredString = String::from("FizzBuzz!").green();

    for i in 1u8..=100 {
        let div_3: u8 = i % 3;
        let div_5: u8 = i % 5;

        match (div_3, div_5) {
            (0, 0) => println!("{}", fizz_buzz),
            (0, _) => println!("{}", fizz),
            (_, 0) => println!("{}", buzz),
            (_, _) => println!("{}", i),
        }
    }
}

fn fizzbuzz_3<'a, T>(elements: T)
where
    T: IntoIterator<Item = &'a u8>,
{
    for j in elements {
        match (j % 3 == 0, j % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", j),
        }
    }
}
