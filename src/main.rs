use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessinng Game! You have 8 attempts!");
    println!("Guess a number between 1 to 100!");
    
    let secret_num = rand::thread_rng()
        .gen_range(1..100);

    let mut limit = 0;

    loop {
        if limit == 8 {
            println!("Guessed 8 times, GAME OVER!");
            
            println!("Play again? y/n");
            let mut reset = String::new();

            io::stdin()
                .read_line(&mut reset)
                .expect("Failed to read the line");

            match reset.trim().to_lowercase().as_str() {
                "yes" | "y" => {
                    limit = 0;
                    continue;
                }
                _ => {
                    println!("Thank you for playing ありがとう！！　―sceiiya dev");
                    break;
                },
            }
        }

        let atmpt = 8 - limit; 
        println!("Please Input your Number to guess! {atmpt} attempt left.");
        limit += 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess : i32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    continue;
                },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("You Just WON!");
                
                println!("Play again? y/n");
                let mut reset = String::new();

                io::stdin()
                    .read_line(&mut reset)
                    .expect("Failed to read the line");

                // if reset.trim() == "yes" || reset.trim() == "y" {
                //    limit = 0;
                //    continue;
                // }else{
                //    break;
                // }
                
                match reset.trim().to_lowercase().as_str() {
                    "yes" | "y" => {
                        limit = 0;
                        continue;
                    }
                    _ => { 
                        break;
                    },
                }
            },
        }
    }

}
