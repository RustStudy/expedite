# Rust Common toolset

### Time

```rust
extern crate expedite;
use expedite::datetime::period::Period;
use expedite::datetime::time::Time;

fn main() {
    let time = Time::now();

    println!("{:?}", time); // Time { date: Date { year: 2017, month: 7, day: 20 }, hours: 16, minutes: 50, seconds: 5, nanos: 470133000 }

    println!(" {:?}", 2.days().from_now()); // Time { date: Date { year: 2017, month: 7, day: 22 }, hours: 16, minutes: 50, seconds: 5, nanos: 470775000 }

    println!(" {:?}", 2.weeks().from_now()); // Time { date: Date { year: 2017, month: 8, day: 3 }, hours: 16, minutes: 50, seconds: 5, nanos: 470798000 }

    println!(" {:?}", 2.months().from_now()); // Time { date: Date { year: 2199522, month: 2, day: 26 }, hours: 16, minutes: 50, seconds: 5, nanos: 470803000 }

    println!(" {:?}", 2.years().from_now()); // Time { date: Date { year: 2019, month: 7, day: 20 }, hours: 16, minutes: 50, seconds: 5, nanos: 470808000 }

    let mut map = hash!{'{' => '}', '[' => ']', '(' => ')'};
    println!("{:?}", map);
}
```


### Macros

1. Hash Literal

```rust
#[macro_use]
extern crate expedite;

fn main() {
  let mut map = hash!{'{' => '}', '[' => ']', '(' => ')'};
  println!("{:?}", map);
}
```
