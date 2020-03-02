use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashSet;

fn guessnum(){
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101); 
    println!("Please input your guess.");
    let mut scores = HashSet::new();

    scores.insert(1);
    scores.insert(2);
    let mut bible = HashSet::new();
    bible.insert(2);

    let d = scores.intersection(&bible);
    let x = d.count();
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too smale"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
    
}