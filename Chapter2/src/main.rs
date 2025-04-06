use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number is back kids");
    let sec = rand::thread_rng().gen_range(1..101);
    loop
    {
        println!("Enter a number");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", input);

        match input.cmp(&sec)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
