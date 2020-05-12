extern crate rand;
use std::io;
use std::io::Read;
use rand::distributions::{Distribution, Uniform};
use std::{thread, time};

//function to return a random integer in the range from 0 to upper_bound, excluding upper bound
fn random_integer_in_range(upper_bound: usize) -> usize{
    let range = Uniform::from(0..(upper_bound - 1));
    let mut rng = rand::thread_rng();
    let num: usize = range.sample(&mut rng);
    num
}

//function that will read dicitionary.txt into a vector of strings and randomly select one of the strings as the secret guess
fn get_random_word() -> String{
    //reading file into a string buffer
    let mut file = std::fs::File::open("dictionary.txt").unwrap();
    let mut raw_text = String::new();
    file.read_to_string(&mut raw_text).unwrap();
    let split = raw_text.split("\n");
    let string_list: Vec<&str> = split.collect();
    
    //generating a random number as the index of the word to be selected
    let random_int = random_integer_in_range(string_list.len());

    String::from(string_list[random_int])
}

//function that will print the hangman in 6 phases
fn print_man (misses: i32){
    let image_phases : [&str ; 24] = [
    "                     ----------------------------",
    "                     |                          |",
    "                     |                          |",
    "                     |                          |",
    "                ------------                    |",
    "               /            \\                   |",
    "              /   -      -   \\                  |",
    "             |   |x|    |x|   |                 |",
    "              \\              /                  |",
    "               \\    ----    /                   |",
    "                ------------                    |",
    "                    |  |                        |",
    "                    |  |                        |",
    "                   /    \\                       |",
    "                  / /||\\ \\                      |",
    "                 / / || \\ \\                     |",
    "                /||\\ || /||\\                    |",
    "                     ||                         |",
    "                    //\\\\                        |",
    "                   //  \\\\                       |",
    "                __//    \\\\__                    |",
    "               |___|    |___|                   |",
    "                                                |",
    "                 YOU LOSE!!!        ____________|____________"];

    let mut i = 0;
    for x in image_phases.iter(){
        if i < 4 * misses{
            println!("{}", x);
        }
        else{
            println!("");
        }
        i+= 1;
    }
}

//this unit struct will be used to return an error that this character is not alphabetic
struct NonAlphabeticError;

//this function returns the lowercase version of the first character in a string
//used to parse the input from the user
fn str_to_lowercase_char(input: &str) -> Result<char, NonAlphabeticError> {
    let mut iter = input.chars();
    let c = iter.next().unwrap();
    if c.is_alphabetic() {
        Ok(c.to_lowercase().next().unwrap())
    }
    else{
        Err(NonAlphabeticError)
    }
    
}

//helper function that will iterate through the guessed letters and return true if the character is contained
fn str_contains(s: &str, c: char) -> bool{
    let iter = s.chars();
    for ch in iter{
        if ch == c{
            return true
        }
    }
    false
}

//this function will be used to check whether the guessed character is contained within the secret word 
//if so, the character will be inserted at the corresponding indices in progress and then return true
//if the character is not in the string, we will return false
fn update_progress(secret: &str, progress: &mut String, c: char)-> bool{
    //this counter will count the number of matches we have found within the secret string
    let mut counter: usize = 0;
    //initializing an str that contains c and will be used to replace the ranges in the progress string
    let mut c_as_string = String::new();
    c_as_string.push(c);
    //initializing an iterator containing the indices and characters in the secret string
    let char_indices = secret.char_indices();
    for index in char_indices{
        match index{
            (index_num, index_char) =>  {
                if index_char == c{
                    //we have found a match in the secret string, so we will do the appropriate replacement and increment the counter
                    counter += 1;
                    progress.replace_range(index_num..(index_num+1), &c_as_string);
                }
            }
        };
    }

    //returning appropriate values based on the value of counter
    if counter > 0{
        true
    }
    else{
        false
    }
}

fn main() {
    //initializing the secret string
    let secret:String = get_random_word();
    let secret_length = secret.len();
    //creating a string to contain the current guess
    let mut progress = String::new();
    for _ in 0..secret_length{
        progress.push('_');
    }

    let mut misses = 0;
    //string that will hold input strings from the player
    let mut input = String::new();
    let mut c: char = '1';

    let mut letters_guessed = String::new();

    //standard duration used for sleeping
    let wait_time = time::Duration::from_millis(750);

    //main control loop
    while misses < 6{
        //setting up the screen with the current progress and the status of the hangman diagram
        print_man(misses);
        println!("Letters guessed so far: {}", letters_guessed);
        println!("Progress so far: {} ({} letters)", progress, progress.len());

        //uncomment line to see secret word revealed
        //println!("Secret: {}", secret);

        //getting guess from player
        while input.len() != 1{
            println!("Enter a single letter:");
            io::stdin().read_line(&mut input).unwrap();
            input = String::from(input.trim_end());
            if input.len() == 1{
                match str_to_lowercase_char(&input){
                    Ok(ch) => {
                        if str_contains(&letters_guessed, ch){
                            println!("This letter has already been entered.");
                            input = String::new();
                            continue;
                        }
                        else{
                            c = ch;
                            break;
                        }
                    }
                    Err(NonAlphabeticError) => {
                        println!("Please enter an alphabetic character.");
                        input = String::new();
                        continue;
                    }
                }
            }
            else{
                input = String::new();
            }
        }


        //updating the progress string and incrementing misses if the guess was incorrect
        if !update_progress(&secret, &mut progress, c){
            misses += 1;
            println!("Sorry, your guess was wrong.");
        }
        else{
            println!("Good guess!!!");
        }
        input = String::new();
        letters_guessed.push(c);
        if progress == secret{
            break;
        }
        thread::sleep(wait_time);
    }

    //evaluating whether the player has won or not
    if misses < 6{
        println!("Congratulations, you won!!!");
    }
    else{
        print_man(misses);
        println!("Secret word was: {}", secret);
    }
}
