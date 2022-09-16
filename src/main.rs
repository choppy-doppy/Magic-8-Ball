use std::io;
use std::num::{ParseIntError};
use rand::seq::SliceRandom;

fn main() {
    println!("ask a question");

    loop {

        // this line will accept user input and set it as a variable named 'question'
        let mut question = String::new();

        // this block will read the line and issue an error message if it can't read the line
        io::stdin()
            .read_line(&mut question)
            .expect("failed to read line");

        // this will check if the input is a number and issue an integer error
        let question:Result<i128, ParseIntError> = question.trim().parse();

        // if the integer error is issued then 'question' will not be ok and it will print the if statement
        // if it is okay then it will go ahead with the else statement
        if question.is_ok() {
            println!("that's a number not a question");
        } else { // this whole if else sets up a vector of a few different options and then the randomly chooses one of them and prints it out
            let answers = vec![
                "yes", "no", "maybe",
                "what the f***??", "that's a stupid question",
                "hell no", "HELL YEAAHH!!", "possibly",
                "perchance", "what is wrong with you",
                "????", "to be honest, i don't really know",
                "you're an idiot",
            ];
            // after the vector this chooses a random part and outputs it
            let mut rng = rand::thread_rng();
            let choice = answers.choose(&mut rng).unwrap();
            println!("{choice}");
        }
    }
}

