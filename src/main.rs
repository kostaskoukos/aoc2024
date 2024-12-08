mod day1;
use crate::day1::run_day1;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("Must provide a day of the challenge (dayN)");
    println!("Running {}", day);

    match day.as_str() {
        "day1" => println!("{:?}", run_day1()),
        _ => panic!("Couldn't indentify day!"),
    }
}
