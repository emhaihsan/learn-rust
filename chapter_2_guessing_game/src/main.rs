use std::cmp::Ordering;
use std::io; // import modul

use rand::Rng; // gunakan modul random

fn main() {
    println!("Guess the number!"); // cetak di console

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess."); // cetak di console permintaan

        let mut guess = String::new(); // bikin variabel guess

        io::stdin()
            .read_line(&mut guess) // baca input dari user
            .expect("Failed to read line"); // error handling

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}