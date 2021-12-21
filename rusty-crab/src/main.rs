use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let secret = rand::thread_rng().gen_range(1..100);
    //println!("Secret: {}",secret);

    loop{
        println!("Enter a number: ");

        println!("Input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
        // shadowing allows us to use the same variable name with different     type

        // .expect() or match
        let num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed {}",guess);
        match num.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}