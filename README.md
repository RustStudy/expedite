# Rust Common toolset

### Macros

1. Hash Literal

```
#[macro_use]
extern crate expedite;

fn main() {
  let mut map = hash!{'{' => '}', '[' => ']', '(' => ')'};
  println!("{:?}", map);
}
```
