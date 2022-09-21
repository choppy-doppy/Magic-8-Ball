#![feature(option_result_contains)]

use std::io;
use std::num::ParseIntError;
use rand::seq::SliceRandom;

fn main() {
    println!("ask a question");
    println!("when you're done, type 'exit' to exit");

    loop {

        // this line will accept user input and set it as a variable named 'question'
        let mut question = String::new();

        // this block will read the line and issue an error message if it can't read the line
        io::stdin()
            .read_line(&mut question)
            .expect("failed to read line");

        // this block will check if you typed "exit" and when you do exit the program
        match question.trim() {
            "exit" => std::process::exit(0),
            _ => {}
        }

        // this will check if the input is a number and issue an integer error
        let question:Result<i128, ParseIntError> = question.trim().parse();

        // if the integer error is issued then 'question' will not be ok and it will print the if statement
        // if it is okay then it will go ahead with the else statement
        if question.is_ok() {
            println!("that's a number not a question");
        } else {
            let answers = vec![
                "yes", "no", "maybe",
                "what the f***??", "that's a stupid question",
                "hell no", "HELL YEAAHH!!", "possibly",
                "perchance", "what is wrong with you",
                "????", "to be honest, i don't really know",
                "you're an idiot", "how peculiar", "why do you ask",
                "you're a weird one eh?", "eh?", "eh", "how aboot we go get some timmies then we'll talk later eh?",
                "cum",
            ];
            // after the vector this chooses a random part and outputs it
            let mut rng = rand::thread_rng();
            let choice = answers.choose(&mut rng).unwrap();
            println!("{choice}");

        };
    }
}

