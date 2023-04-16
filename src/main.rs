#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;
use colored::*;

const options: [&str; 3] = ["rock", "paper", "scissors"];

fn winning_conditions() -> HashMap<&'static str, &'static str> {
    let pairs = [("rock", "paper"), ("paper", "scissors"), ("scissors", "rock"), ("gun", "to surrender")];
    pairs.iter().cloned().collect()
}

fn computer_intelligence<'a>(difficultySettings: i32, playerChoice: &'a str, rng: &'a mut impl Rng) -> &'a str {
    let winPlay = winning_conditions();

    match difficultySettings {
        1 => options[rng.gen_range(0..3)],
        2 => if rng.gen_range(0..10) % 3 == 0 {
                winPlay[playerChoice]
            } else {
                options[rng.gen_range(0..3)]
            },
        3 => winPlay[playerChoice],
        _ => unreachable!(),
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let winPlay = winning_conditions();

    let gunDurability = rng.gen_range(0..101);
    let mut gunHealth: i32 = gunDurability;

    let border = "----------------------------------------------\n";

    println!("{0}-- {1} --\n{0}", border, "Fun Rock Paper Scissors Game against AI!".yellow());

    let difficultySettings: i32 = loop {
        print!("How confident are you in your brain power? {}\n> ", "(1/2/3)".green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input.parse() {
            Ok(num) if num >= 1 && num <= 3 => break num,
            _ => println!("{}", "Invalid input.".red()),
        }
    };

    loop {
        print!("\n{}\n> ", "Rock, Paper, Scissors!".cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let playerChoice = input.trim().to_lowercase();

        if !options.contains(&playerChoice.as_str()) && playerChoice != "gun" {
            println!("{}", "Invalid input.".red());
            continue;
        }

        let computerChoice = computer_intelligence(difficultySettings, &playerChoice, &mut rng);
        println!("Computer chooses {}.\n", computerChoice.bright_magenta());

        if playerChoice == "gun" {
            gunHealth -= 30;

            if gunHealth <= 0 {
                println!("{}\n{} (your life too)...\n", "Your gun overheats and the bullet explodes in your hand!".white().on_red(), "You lost".red())
            } else {
                if gunHealth - 30 <= 0 {
                    println!("{}", "You feel something is heating up...".bright_black())
                } else {
                    println!("Gun defeats all!");
                }

                println!("{}\n", "You win!".bright_green())
            }

        } else {
            if playerChoice == computerChoice {
                println!("{}\n", "It's a draw!".yellow());
            } else if winPlay[&playerChoice.as_str()] == computerChoice {
                println!("{}\n", "You lost...".red());
            } else {
                println!("{}\n", "You win!".bright_green());
            }
        }

        let repeatChoice = loop {
            print!("Play again? {}\n> ", "(Y/N)".green());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let repeatChoice = input.trim().to_lowercase();

            if repeatChoice == "y" || repeatChoice == "n" {
                break repeatChoice;
            } else {
                println!("{}", "Invalid input.".red());
            }
        };
    
        if repeatChoice == "n" || gunHealth <= 0 {
            if gunHealth <= 0 {
                println!("\n{}", "You're already dead, so you can't continue the game...".white().on_red());
            }

            break;
        }
    }

    println!("\n{0}----------- {1} -----------\n{0}", border, "Thank you for playing!".yellow());
}
