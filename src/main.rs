#[macro_use]
extern crate expedite;

use expedite::datetime::period::Period;
use expedite::datetime::time::Time;

fn main() {
    let time = Time::now();

    println!("{:?}", time);
    println!("{:?}", 2.days().from_now());
    println!("{:?}", 2.weeks().from_now());
    println!("{:?}", 2.months().from_now());
    println!("{:?}", 2.years().from_now());


    let mut map = hash!{'{' => '}', '[' => ']', '(' => ')'};
    println!("{:?}", map);
}
