[![crates.io](https://img.shields.io/crates/v/dish.svg)](https://crates.io/crates/dish)
[![License](https://img.shields.io/crates/l/dish.svg)](https://choosealicense.com/licenses/mpl-2.0/)
[![Documentation](https://img.shields.io/docsrs/dish/latest)](https://docs.rs/dish)

# dish

_**dish**_ provides several extensions for Rust's libcore.

##  Examples:

### Find greatest count divisor of minimal and maximal numbers in a vector of usize:

```rust
use dish::prelude::*;

fn mmgcd(src: Vec<usize>) -> usize {
    src.into_iter()
       .min_max_pipe(Number::gcd)
       .unwrap()
}
```

### Find sum of all digits of a number:

```rust
use dish::prelude::*;

fn dgsum<T: Number>(n: T) -> u8 {
    n.digits()
     .sum()
}
```