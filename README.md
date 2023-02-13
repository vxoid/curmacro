# Curmacro
Crate with usefull macros like struct getters and setters creation macros

## Links
- [x] docs.rs - https://docs.rs/curmacro
- [x] github - https://github.com/CURVoid/curmacro.git

## Example
```rust
use curmacro::*;

struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    getters!(
        pub get_x(x) -> i32;
        pub get_y(y) -> i32;    
    );
    setters!(
        pub set_x(i32) -> x;
        pub set_y(i32) -> y;    
    );
}

let x = 13;
let y = 215;
let mut point = Point { x, y };

assert_eq!(point.get_x().clone(), x);
assert_eq!(point.get_y().clone(), y);
let x = 993;
let y = 37;

point.set_x(x);
point.set_y(y);

assert_eq!(point.get_x().clone(), x);
assert_eq!(point.get_y().clone(), y);
```