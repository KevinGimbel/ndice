use regex::Regex;
use std::num::ParseIntError;

pub mod structs;

use structs::{Dice, DiceModifier, DiceResult, RollingHand, RollingHandResult};

const DICE_REGEX: &str = r#"(?P<numberOfDice>[^[dwe\s+-]]+\d?)(?P<explodingDice>[e])?(?P<diceIndicator>[dw])(?P<numberOfSides>\d+)(?P<mod>[+-]\d+)?"#;

fn str_to_i64(input: &str) -> Result<i64, ParseIntError> {
    input.parse()
}

fn parse_dice_from_str(input: &str) -> Option<regex::Captures<'_>> {
    let rx = Regex::new(DICE_REGEX).unwrap();

    rx.captures(input)
}

pub fn parse_dices(args: Vec<String>) -> Result<RollingHand, ParseIntError> {
    let mut hand = RollingHand::default();

    for arg in args {
        let res = parse_dice_from_str(arg.as_str());

        match res {
            Some(matches) => {
                let mut i: i64 = 0;
                let number_of_dice = matches.name("numberOfDice").unwrap().as_str();
                let number_of_sides = matches.name("numberOfSides").unwrap().as_str();
                let exploding_dice = match matches.name("explodingDice") {
                    Some(_ed) => true,
                    None => false,
                };
                let modifier = match matches.name("mod") {
                    Some(ed) => ed.as_str(),
                    None => "",
                };
                let modifier_value: i64 =
                    match modifier.split(&['+', '-']).collect::<Vec<&str>>().last() {
                        Some(v) => match str_to_i64(v) {
                            Ok(v) => v,
                            Err(_e) => 0,
                        },
                        _ => 0,
                    };
                let dices = str_to_i64(number_of_dice)?;
                let sides = str_to_i64(number_of_sides)?;
                // match minimum number on dice based on dice type
                let min: i64 = match sides {
                    10 => 0,
                    _ => 1,
                };
                while i < dices {
                    i = i + 1;
                    hand.dices.push(Dice {
                        min,
                        sides,
                        modifier: DiceModifier {
                            is_plus: modifier.contains("+"),
                            value: modifier_value,
                        },
                        exploding_dice,
                    })
                }
            }
            _ => {}
        }
    }

    Ok(hand)
}

pub fn roll_dices(in_hand: RollingHand) -> RollingHandResult {
    let hand = in_hand;
    let mut result = RollingHandResult::default();

    for dice in hand.dices {
        let dice_result = dice.roll();
        let mut modi = "-";
        let mut exploding = "";
        if dice.modifier.is_plus {
            modi = "+";
        };

        if dice.exploding_dice {
            exploding = "e";
        }

        result.rolls.push(DiceResult {
            sides: dice.sides,
            result: dice_result,
            modifier: format!("{}{}", modi, dice.modifier.value),
            exploding_dice: String::from(exploding),
        });
        result.sum += dice_result;
        if dice.modifier.is_plus {
            result.sum += dice.modifier.value
        } else {
            result.sum -= dice.modifier.value
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice() {
        struct TestCase<'a> {
            name: &'a str,
            input: &'a str,
            expected: RollingHand,
        }

        let tests = [
            TestCase {
                name: "dice: 1d4",
                input: "1d4",
                expected: RollingHand {
                    dices: vec![Dice {
                        sides: 4,
                        min: 1,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            },
            TestCase {
                name: "dice: 3d12",
                input: "3d12",
                expected: RollingHand {
                    dices: vec![
                        Dice {
                            sides: 12,
                            ..Default::default()
                        },
                        Dice {
                            sides: 12,
                            ..Default::default()
                        },
                        Dice {
                            sides: 12,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            },
            TestCase {
                name: "dice: 1ed6",
                input: "1ed6",
                expected: RollingHand {
                    dices: vec![Dice {
                        sides: 6,
                        exploding_dice: true,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            },
            TestCase {
                name: "dice: 1w6",
                input: "1w6",
                expected: RollingHand {
                    dices: vec![Dice {
                        sides: 6,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            },
            TestCase {
                name: "dice: 1d10",
                input: "1d10",
                expected: RollingHand {
                    dices: vec![Dice {
                        sides: 10,
                        min: 0,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            },
        ];

        for test in tests {
            dbg!(test.name);
            let dice = parse_dices(vec![String::from(test.input)]).unwrap();
            assert_eq!(test.expected, dice)
        }
    }
}
