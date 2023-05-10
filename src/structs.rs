use rand::{thread_rng, Rng};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Dice {
    pub sides: i64,
    pub min: i64,
    pub modifier: DiceModifier,
    pub exploding_dice: bool,
}

impl Dice {
    pub fn roll(&self) -> i64 {
        let mut rng = thread_rng();
        let mut n: i64;
        if self.exploding_dice {
            let mut last_result = rng.gen_range(self.min..=self.sides);
            n = last_result;
            while last_result != self.sides {
                println!("last_result = {}", last_result);
                last_result = rng.gen_range(self.min..=self.sides);
                n += last_result
            }
        } else {
            n = rng.gen_range(self.min..=self.sides);
        }
        n
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self {
            sides: 0,
            min: 1,
            modifier: DiceModifier {
                is_plus: false,
                value: 0,
            },
            exploding_dice: false,
        }
    }
}

#[derive(Debug)]
pub struct DiceModifier {
    pub is_plus: bool,
    pub value: i64,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct RollingHand {
    pub dices: Vec<Dice>,
    pub result: RollingHandResult,
}

impl PartialEq for RollingHand {
    fn eq(&self, other: &Self) -> bool {
        // TODO: maybe this isn't the best implementation 😬
        return format!("{:?}", self.dices) == format!("{:?}", other.dices);
    }

    fn ne(&self, other: &Self) -> bool {
        // TODO: maybe this isn't the best implementation 😬
        return format!("{:?}", self.dices) != format!("{:?}", other.dices);
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct RollingHandResult {
    pub sum: i64,
    pub rolls: Vec<DiceResult>,
}

impl Default for RollingHandResult {
    fn default() -> Self {
        Self {
            sum: 0,
            rolls: vec![],
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct DiceResult {
    pub sides: i64,
    pub result: i64,
    pub modifier: String,
    pub exploding_dice: String,
}

impl fmt::Display for DiceResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}d{} => {} ({})",
            self.exploding_dice, self.sides, self.result, self.modifier
        )
    }
}

impl fmt::Display for RollingHandResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rolls: Vec<String> = vec![];
        for dice in &self.rolls {
            rolls.push(format!("{}", dice));
        }
        write!(f, "Rolls: {:?}\nResult: {}", rolls, self.sum)
    }
}

impl Default for RollingHand {
    fn default() -> Self {
        Self {
            dices: vec![],
            result: RollingHandResult::default(),
        }
    }
}
