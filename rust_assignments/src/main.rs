pub mod temp_convert;
pub mod num_analyzer;
pub mod guessing_game;
pub mod m2_04_a1;
pub mod m2_04_a2;

// uncomment out if want to run all together 
//and comment out whichever if needed to run seperately
fn main() {
    /*
    println!("Temperature Conversion");
    temp_convert::main();

    println!("Number Analyzer");
    num_analyzer::main();

    println!("\nGuessing Game");
    guessing_game::main();
    
    println!("Assignment 1: Mutable Reference Sum with Step");
    m2_04_a1::main();
    */
    println!("\nAssignment 2: Word Frequency Counter");
    m2_04_a2::main();
}
