use std::fmt;

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Dice {
    pub sides: u64,
    pub min: u64,
}

impl Dice {
    pub fn roll(&self) -> u64 {
        let mut rng = thread_rng();
        let n: u64 = rng.gen_range(self.min..self.sides);
        n
    }
}

#[derive(Debug)]
pub struct RollingHand {
    pub dices: Vec<Dice>,
    pub result: RollingHandResult,
}

#[derive(Debug)]
pub struct RollingHandResult {
    pub sum: u64,
    pub rolls: Vec<DiceResult>,
}

impl Default for RollingHandResult {
    fn default() -> Self {
        Self{sum: 0, rolls: vec![]}
    }
}

#[derive(Debug)]
pub struct DiceResult {
    pub sides: u64,
    pub result: u64,
}

impl fmt::Display for DiceResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d{} => {}", self.sides, self.result)
    }
}


impl fmt::Display for RollingHandResult {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rolls: Vec<String> = vec![];
        for dice in &self.rolls {
            rolls.push(format!("d{} => {}", dice.sides, dice.result));
        }
        write!(f, "Rolls: {:?}\nResult: {}", rolls, self.sum)
    }
}

impl Default for RollingHand {
    fn default() -> Self {
        Self{dices: vec![], result: RollingHandResult::default()}
    }
}
