use std::io::{stdin, stdout, Write, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};
use random::Source;

const MAX_ATTEMPTS: usize = 3;
const LOW: usize = 1;
const HIGH: usize = 100;

fn main() {
    println!("I generated a Random Number from {LOW} to {HIGH}.");
    println!("You have {MAX_ATTEMPTS} attempts to guess it.");
    let mut source = random::default(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let mut answer: usize = source.read::<usize>()%(HIGH - LOW + 1) + LOW;
    let mut attempt = 0;
    'next: loop {
        if attempt < MAX_ATTEMPTS {
            print!("[{attempt}/{MAX_ATTEMPTS}] ", attempt = attempt + 1);
            stdout().flush().unwrap();
            let mut guess = String::new();
            match stdin().read_line(&mut guess) {
                Ok(_) => {},
                Err(err) => match err.kind() {
                    ErrorKind::InvalidData => {
                        println!("Stop typing gibberish and play the game already ._.");
                        continue 'next;
                    }
                    _ => panic!("Something went wrong: {err}"),
                }
            }
            match guess.trim().parse::<usize>() {
                Ok(guess) => if guess == answer {
                    println!("Yes! It was {answer}! You won!");

                    print!("Try again? [Y/n] ");
                    stdout().flush().unwrap();
                    let mut yorn = String::new();
                    match stdin().read_line(&mut yorn) {
                        Ok(_) => {},
                        Err(err) => match err.kind() {
                            ErrorKind::InvalidData => {
                                answer = source.read::<usize>()%(HIGH - LOW + 1) + LOW;
                                attempt = 0;
                                println!("I generated a Random Number from {LOW} to {HIGH}.");
                                println!("You have {MAX_ATTEMPTS} attempts to guess it.");
                                continue 'next;
                            }
                            _ => panic!("Something went wrong: {err}"),
                        }
                    }
                    match yorn.trim().to_uppercase().as_str() {
                        "N" | "NO" => break,
                        _ => {
                            answer = source.read::<usize>()%(HIGH - LOW + 1) + LOW;
                            attempt = 0;
                            println!("I generated a Random Number from {LOW} to {HIGH}.");
                            println!("You have {MAX_ATTEMPTS} attempts to guess it.");
                        }
                    }
                } else if guess < answer {
                    println!("No! The answer is ᵇᶦᵍᵍᵉʳ!");
                    attempt += 1;
                } else if guess > answer {
                    println!("No! The answer is ₛₘₐₗₗₑᵣ!");
                    attempt += 1;
                }
                Err(_) => {
                    println!("This is not even a number, come on ._.");
                    attempt += 1;
                }
            }
        } else {
            println!("Sorry you ran out of attempts! The answer was {answer}.");
            print!("Try again? [Y/n] ");
            stdout().flush().unwrap();
            let mut yorn = String::new();
            match stdin().read_line(&mut yorn) {
                Ok(_) => {},
                Err(err) => match err.kind() {
                    ErrorKind::InvalidData => {
                        answer = source.read::<usize>()%(HIGH - LOW + 1) + LOW;
                        attempt = 0;
                        println!("I generated a Random Number from {LOW} to {HIGH}.");
                        println!("You have {MAX_ATTEMPTS} attempts to guess it.");
                        continue 'next;
                    }
                    _ => panic!("Something went wrong: {err}"),
                }
            }
            match yorn.trim().to_uppercase().as_str() {
                "N" | "NO" => break,
                _ => {
                    answer = source.read::<usize>()%(HIGH - LOW + 1) + LOW;
                    attempt = 0;
                    println!("I generated a Random Number from {LOW} to {HIGH}.");
                    println!("You have {MAX_ATTEMPTS} attempts to guess it.");
                }
            }
        }
    }
}
