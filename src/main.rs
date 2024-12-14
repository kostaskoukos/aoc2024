mod day1;
use crate::day1::run_day1;

mod day2;
use crate::day2::run_day2;

mod day3;
use crate::day3::run_day3;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("Must provide a day of the challenge (dayN)");
    println!("Running {}", day);

    match day.as_str() {
        "day1" => println!("{:?}", run_day1()),
        "day2" => println!("{:?}", run_day2()),
        "day3" => println!("{:?}", run_day3()),
        _ => panic!("Couldn't indentify day!"),
    }
}
