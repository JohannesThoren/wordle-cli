mod guess;
use color_print::cprint;
use guess::Response;
use core::time;
use std::io::{stdin, stdout, Read, Write};
use std::thread::{self, sleep};

const DELAY: u64 = 150;

fn main() -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    let mut correct = false;
    for n in 1..6 {
        let mut input = read_line();

        while input.len() != 5 {
            println!("Word must contain exactly 5 characters!");
            input = read_line();
        }

        let response = guess::guess(&client, &input)?;

        check_guess_response(&response);

        match response.was_correct {
            Some(wc) => {
                if wc == true {
                    println!("Correct! word of the day was {}", response.guess);
                    correct = true;
                    break;
                }
            }
            None => todo!(),
        }

        stdout().flush();
    }

    if !correct {
        println!("You lost! Go to https://wordle-api.vercel.app/ to get word of the day",);
    }

    Ok(())
}

fn check_guess_response(response: &guess::Response) {
    match &response.character_info {
        Some(ci) => {
            for i in ci {
                if i.scoring.in_word && i.scoring.correct_idx {
                    cprint!("<g>{}", i.char)
                } else if i.scoring.in_word && !i.scoring.correct_idx {
                    cprint!("<y>{}", i.char)
                } else if !i.scoring.in_word {
                    cprint!("<r>{}", i.char)
                }
                stdout().flush();
                thread::sleep(time::Duration::from_millis(DELAY));
            }
        }
        None => {
            if !response.was_correct.unwrap() {
                eprintln!("something went wrong!");
            }
            else {
                for r in response.guess.chars() {
                    cprint!("<g>{}", r);
                    stdout().flush();
                    thread::sleep(time::Duration::from_millis(DELAY));
                }
            }
        }
    }
    print!("\n");
}

fn read_line() -> String {
    print!("guess: ");
    stdout().flush();
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line.pop();
    line
}
