//initializing an array that contains the stages of the hangman image that will be printed


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
    print_man(6);
}
