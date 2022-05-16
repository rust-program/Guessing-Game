use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_no = rand::thread_rng().gen_range(1..101);

    println!("The secret number is : {}",secret_no);

    loop{

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 =match guess.trim().parse() //parse string to integer
    {
        Ok(num)=>num,
        Err(_) => continue,
    };
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_no)
    {
        Ordering::Less => println!("Your Guess is Too Small"),
        Ordering::Equal => { println!("Congratulations You Win");
                            break;}
        Ordering::Greater =>println!("Your Guess is Too Big"),
    }
}
}
