# NOTES

##Common concepts

### Variables and Mutability

Variables are immutable by default for safety and concurrency. You can opt out and make them mutable (using `mut`).
Considerations for immutability vs mutability include time taken to copy and return newly allocaed instances when dealing with large data structures when using immutables and ability to write functional code.

Constants cant have `mut`; always immutable.

#### Variable shadowing

declaring a new variable with the same name as a previous one by repeating the use of the `let` keyword
Shadowing allows us to change the type of a variable as opposed to using `mut`
EX:

```
    let spaces = "   ";
    let spaces = spaces.len();
```

vs

```
    let mut spaces = "   ";
    spaces = spaces.len(); // Will throw an error
```

### Data types

#### Scalar Types

Integers: `i` for signed, `u` for unsigned, there 8, 16, 32, 64, 128 and arch bit size integers (arch size referring to you having a 32 bit vs 64 bit arch computer).

Floating Points: `f32` and `f64`. Default type is `f64` because its roughly the same speed but more precision

Boolean: `bool`

Character: `char`, 4 bytes and represents Unicode Scalar Value.

#### Compound Types

Compound types group multiple values into a single type.

TUPLES: grouping a number of values of different types into one fixed length type. Once declared, cannot grow or shrink.
`let tup: (i32, f64, u8) = (500, 6.4, 1);`
Tuples can be destructured as well:
`let (x, y, z) = tup;`
You can access the elements using `.` notation and the index
`let five_hundred = tup.0`

ARRAYS: a number of elements of the same type. Arrays in Rust have a fixed length like tuples.
`let a = [1,2,3]`
Arrays are useful if you want data allocated on the stack rather than the heap. They aren't flexible compared to vector types which are allowed to grow or shrink. Access elements using the `[]` notation like other languages

### Functions

```
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

#### Statements and Expressions!

Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

```
fn main() {
    let y = 6;
}
```

This is a statement > function definitions are statements
This has an important implication: you cant bind to a statement
`let x = (let y = 6)` would be similar to `x = y = 6` in other languages but in Rust would throw an error.

Expressions evaluate to something. Expressions can be part of statements. Calling a function, calling a macro, using `{}` to create new scopes are all expressions.

```
let x = 5;

let y = {
    let x = 3;
    x + 1
};

println!("The value of y is: {}", y);
```

Notice that the final line `x+1` doesn't have a semicolon, it is an expression. Adding a semicolon would turn it into a statement which doesn't return a value.

```
fn five() -> i32 {
    5
}
```

Therefore the above is a valid function that returns the value 5.

### Control Flow

#### if

```
let number = 3;
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

This is not JS so `if number {` is not a valid conditional in the above scenario.

`if` is an expression and can be used in `let`:
`let number = if condition { 5 } else { 6 };`

#### Loops

`loop`, `for` and `while`
`loop` can be used to return a value. Add the value you want after the `break` expression.

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

```
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("the value is: {}", a[index]);

    index += 1;
}
```

```
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```
