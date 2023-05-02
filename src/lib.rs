use regex::Regex;

pub mod ndice;

use ndice::structs::{RollingHand, RollingHandResult, Dice, DiceResult};

const DICE_REGEX: &str = r#"(?P<numberOfDice>[^[dw\s]]+\d?)(?P<diceIndicator>[dw])?(?P<numberOfSides>\d+)"#;

fn str_to_u64(input: &str) -> u64 {
    let du64: u64 = input.parse().unwrap();
    return du64;
}


pub fn parse_dice(args: std::env::Args) -> RollingHand {
    let mut hand = RollingHand::default();
    let rx = Regex::new(DICE_REGEX).unwrap();

    for arg in args {
        let res = rx.captures(&arg.as_str());

        match res {
            Some(matches) => {
                let mut i: u64 = 0;
                let number_of_dice = matches.name("numberOfDice").unwrap().as_str();
                let number_of_sides = matches.name("numberOfSides").unwrap().as_str();
                let dices = str_to_u64(number_of_dice);
                let sides = str_to_u64(number_of_sides);
                // match minimum number on dice based on dice type
                let min: u64 = match sides {
                    10 => 0,
                    _ => 1
                };
                while i < dices {
                    i = i + 1;  
                    hand.dices.push(Dice{min, sides})
                }
            },
            _ => {}
        }
    }

    hand
}

pub fn roll_dices(in_hand: RollingHand) -> RollingHandResult {
    let hand = in_hand;
    let mut result = RollingHandResult::default();

    for dice in hand.dices {
        let dice_result = dice.roll();
        result.rolls.push(DiceResult{sides: dice.sides, result: dice_result});
        result.sum += dice_result;
    }

    result
}
