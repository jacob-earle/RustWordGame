extern crate rand;
use std::io::Read;
use rand::distributions::{Distribution, Uniform};

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

    //main control loop
    while misses < 6{
        misses += 1;
    }
}
