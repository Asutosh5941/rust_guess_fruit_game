use std::io::{self, Write};
use rand::{Rng, thread_rng};
use std::process::Command;
use std::process::ExitStatus;

#[cfg(target_os = "linux")]
use std::os::unix::prelude::ExitStatusExt;

#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt;



fn main() {
    
    // Variables
    let mut quit: u8 = 0;
    let mut contains: bool = false;
    let mut error_count: u8 = 0;
    let mut input: String = String::new();
    let mut rng = thread_rng();
    let mut random_no: usize = rng.gen_range(0..=3);
    let fruits: [&str; 4] = ["apple", "banana", "guava", "grapes"];
    let mut random_fruit: &str = fruits[random_no];
    only_clear_scr();
    println!("Welcome to guess the fruit game !!");
    println!("");
    println!("Fruits list: {:?}", fruits);
    println!("To exit the game enter 'quit'");
    println!("");
    
    loop {
        clear_scr();
        // Taking user input
        input.clear();
        if contains {
            contains = false;
        }
        println!("Enter your guess: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().to_lowercase() == "quit" || input.trim().to_lowercase() == "q" || quit == 1 {
                    print!("\n\n");
                    println!(".... Thank you for playing the game ....");
                    quit = 1;
                    break;
                }
                else if quit == 0 {
                    for fruit in &fruits {
                        if input.trim().to_lowercase().contains(fruit) || contains == true {
                            contains = true;
                        }
                        else {
                            contains = false;
                        }
                    }
                }
                if !contains {
                    print!("\nInvalid Input !!\n\n");
                    continue;
                }
                println!("");
                println!("Random Fruit: {}", random_fruit);
                println!("Your Input: {}", input.trim().to_lowercase());
                if input.trim().to_lowercase() == random_fruit && contains == true {
                    print!("\n");
                    println!("You Win !!");
                    print!("\n");
                    random_no = rng.gen_range(0..=3);
                    random_fruit = fruits[random_no];
                    if error_count != 0 {
                        error_count = 0;
                    }
                }
                else if input.trim().to_lowercase() != random_fruit && contains == true {
                    print!("\n");
                    println!("You Lose !!");
                    error_count += 1;
                    print!("\n");
                    if error_count == 3 {
                        print!("\n\n\t You Finally LOSE !!\n\n");
                        break;
                    }
                    random_no = rng.gen_range(0..=3);
                    random_fruit = fruits[random_no];
                }
                input.clear();
            }
            Err(error) => {
                println!("Error in taking input.");
                println!("Error code: {}", error);
            }
        }
    }
}

fn clear_scr() {
    let mut string: String = String::new();
    let mut status: ExitStatus = ExitStatusExt::from_raw(1);
    println!("");
    println!("Press any key to continue...");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut string).expect("Failed to read line");

    if cfg!(target_os = "windows") {
        status = Command::new("cls").status().expect("Failed to execute command");
    }

    else if cfg!(target_os = "linux") {
        status = Command::new("clear").status().expect("Failed to execute command");
    }
    
    if status != ExitStatusExt::from_raw(0) {
        println!("Program exited with status {}", status);
    }
}

fn only_clear_scr() {
    let mut status: ExitStatus = ExitStatusExt::from_raw(1);

    if cfg!(target_os = "windows") {
        status = Command::new("cls").status().expect("Failed to execute command");
    }

    else if cfg!(target_os = "linux") {
        status = Command::new("clear").status().expect("Failed to execute command");
    }
    
    if status != ExitStatusExt::from_raw(0) {
        println!("Program exited with status {}", status);
    }

}
