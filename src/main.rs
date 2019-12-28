// PROGRAM TEBAK ANGKA

// MUAT DEPEDENCIES
use rand::Rng;   // THIRD PARTY - CEK AJA DI Cargo.toml
use std::io;
use std::cmp::Ordering;

fn main() {
    // AMBIL SATU ANGKA ACAK DARI 1-100
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Guess the number!");

    // MASUK KE DALAM PERULANGAN
    loop{
        println!("Please input your guess.");

        // TAMPUNG INPUTAN
        let mut guess = String::new();
        
        // ERROR HANDLING
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("That isn't number");
                continue
            }
        };

        println!("You guessed: {}", guess);

        // CEK INPUTAN
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}