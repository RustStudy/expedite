#[macro_use]
extern crate expedite;

pub use expedite::datetime::period::Period;
pub use expedite::datetime::time::Time;

fn main() {
    let time = Time::now();

    println!("{:?}", time);
    println!("2.days().from_now() : {:?}", 2.days().from_now());
    println!("2.weeks().from_now() : {:?}", 2.weeks().from_now());
    println!("2 months {:?}", 2.months().from_now());
    println!("2 years {:?}", 2.years().from_now());

    let mut map = hash!{'{' => '}', '[' => ']', '(' => ')'};
    println!("{:?}", map);
}
