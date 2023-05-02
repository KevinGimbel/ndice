use ndice;

fn main() {
    let args = std::env::args();
    let dice_in_hand = ndice::parse_dice(args);

    let res = ndice::roll_dices(dice_in_hand);

    println!("{}", res);
}
