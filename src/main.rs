use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Hello! Welcome to my guessing number game.");
    'program_loop: loop{
        let mut correct_guess = rand::thread_rng().gen_range(1..=250);

        loop{
        println!("Please make a guess (Your guess must be a number)");
        let mut user_guess = String::new();

        stdin()
            .read_line(&mut user_guess)
            .expect("Failed to get user input");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

                
        match user_guess.cmp(&correct_guess){
            Ordering::Less => println!("Too small, try again!"),
            Ordering::Greater => println!("Too big, try again!"),
            Ordering::Equal =>{ 
                println!("CONGRATULATIONS, YOU WIN!");
                println!("Will you like to continue? Type 1 to Continue or any other number to end");
                let mut continue_decision = String::new();
                stdin()
                    .read_line(&mut continue_decision)
                    .expect("failed to get user input");
                
                let continue_decision : i32 = match continue_decision.trim().parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if continue_decision == 1{
                    println!("You want to continue");
                }else{
                    println!("Good bye!");
                    break 'program_loop;
                }

                break
            }
            }

        }
    }

   
    

}