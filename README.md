# rust-learning
This repository tracks my Rust language learning.

Commnands:

Start a new cargo project
```rust
cargo new <project-name>
```

Check your program for errors (run this withing the project dir)
```rust
cargo check
```

Build a cargo project. (run this withing the project dir)
```rust
cargo build
```

Run a cargo project. (run this withing the project dir)
```rust
cargo run
```

Format a rust file
```rust
rustfmt <file-path>
```

const, mutable and immutable vars
predule - standard macros and functions that come with rust like: `fn`, `println!`

## Datatypes:

scalar datatypes = int, float, bool, char
compound datatypes = arrays, tuple

#### scalar datatypes
int - default is u32
unsigned integer (can store negative and positive values): int: u8, u16, u32, u64, u128
signed int (only positive): i8, i16, i32, i64, i128

float - f32 and f64 only (default is f64)

bool - true/false, 1/0

char - '<any-char>' //are always in single quotes and sting in double quotes

#### compound datatypes

tuple - immutable collection of elements
let tup: (i32, char, bool) = (5, 'a', true);
accessing a tuple -> tup.0

let array: [i32; <len>] = [<elements>];
let array: [i32; 3] = [1, 2, 3];

### type conversion
Rust by default doesn't do type conversion; if you need to do some arithmetic
operation between var then we would need to do the type conversion
let x = 10u32;
let y = 266_i64;
let z = x as i64 + y

10u32
10_u32
10 as u32

these are the 3 ways of doing it

percidence in logical operators: !, &&, ||

### Concepts

Shadowing means the same variable name used but they differ in datatype
Eg:
let age: u32 = 24;
let age: &str = "24";
here the same variable name `age` is being used as both 32bit int and str

start with `_` for a variable it ignores it to be used in the future

`match`is used as `swtich`

```rust
match age {
    1..=10 => println!("Age between 1 and 10"),
    21 | 32 => println!("Your age is 21 or 32"),
    50..=u32::MAX => println!("Age between 50 and {}", u32::MAX),
}
```

#### Loops:
1. loops {}
2. while <condition> {}
3. for i in arr.iter() 

* Generics
* Traits
* Implementation
* Struct
* module

