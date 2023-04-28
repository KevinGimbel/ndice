use std::env::Args;
use regex::Regex;
use rand::{thread_rng, Rng};

const DICE_REGEX: &str = r#"(?P<numberOfDice>[^[dw\s]]+\d?)(?P<diceIndicator>[dw])?(?P<numberOfSides>\d+)"#;

#[derive(Debug)]
struct RollingHand {
    dices: Vec<Dice>,
}

impl Default for RollingHand {
    fn default() -> Self {
        Self{dices: vec![]}
    }
}

#[derive(Debug)]
struct Dice {
    sides: u64,
    min: u64,
}

fn str_to_u64(input: &str) -> u64 {
    let du64: u64 = input.parse().unwrap();
    return du64;
}


fn parse_dice(args: Args, rx: Regex) -> RollingHand {
    let mut hand = RollingHand::default();
    
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

fn main() {
    let args = std::env::args();
    let rx = Regex::new(DICE_REGEX).unwrap();
    let mut rng = thread_rng();
    let dice_in_hand = parse_dice(args, rx);
    let mut result: u64 = 0;
    let mut rolled: Vec<String> = vec![];
    
    for dice in dice_in_hand.dices {
        // a d1 is a one-sided dice and it's breaking the random generator 😬
        // so the d1 and d0 are special cases
        if dice.sides <= 1 {
            result += dice.sides;
            rolled.push(format!("d{} => {}", dice.sides, dice.sides));
        } else {
            let n: u64 = rng.gen_range(dice.min..dice.sides);
            rolled.push(format!("d{} => {}", dice.sides, n));
            result += n;
        }
    }
    println!("Rolled: {:?}", rolled);
    println!("Result: {:?}", result);
}
