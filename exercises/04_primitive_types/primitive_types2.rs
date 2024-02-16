// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)


fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("{} is Alphabetical!", my_first_initial);
    } else if my_first_initial.is_numeric() {
        println!("{} is Numerical!", my_first_initial);
    } else {
        println!("{} is Neither alphabetic nor numeric!", my_first_initial);
    }

    let your_character = '🔥'; // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("{} is Alphabetical!", your_character);
    } else if your_character.is_numeric() {
        println!("{} is Numerical!", your_character);
    } else {
        println!("{} is Neither alphabetic nor numeric!", your_character);
    }
}
