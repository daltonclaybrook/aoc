mod day3;

use day3::{ determine_power_consumption, determine_life_support_rating };

fn main() {
    let part1 = determine_power_consumption().unwrap();
    let part2 = determine_life_support_rating().unwrap();
    println!("power consumption: {}, life support rating: {}", part1, part2);
}
