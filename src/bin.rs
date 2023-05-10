use std::process::exit;

use ndice;

fn main() {
    // turn args into a Vec<String>
    let args: Vec<String> = Vec::from_iter(std::env::args());
    // parse all dices, this will ignore anything that is not recognized as a dice
    let dice_in_hand = match ndice::parse_dices(args) {
        Ok(dice) => dice,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1)
        }
    };

    // roll the dices
    let res = ndice::roll_dices(dice_in_hand);
    // pretty-print the result
    // this will look like:
    // Rolls: ["1d6 => 5"]
    // Result: 5
    println!("{}", res);
}
