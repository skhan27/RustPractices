# Notes: Part 3

## Structs

Structs are similar to tuples but have named properties.

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Once a struct is defined, we can create an instance of it by supplying the values for that structs fields.

```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user1.email = String.from("another@example.com");
```

In the above, note we made struct mutable and were able to change a field. Rust doesn't allow us to only mark certain fields as mutable.
Like in javascript if you are trying to set a field with a variable and both have the same name, you can only put the name once.
We can also create new instances from other instances or a mix of new data and data from other instances:

```
let user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    ..user1
};
```

Here any field not explicitly set is given the same value as the field in the given instance.

We can also create tuple structs which dont have named fields:

```
struct Color(i32, i32, i32)
let black = Color(0,0,0)
```

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

## Methods

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define the function within the context of Rectangle, we start an impl (implementation) block. Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body.
In the signature for area, we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle due to this method’s being inside the impl Rectangle context. Note that we still need to use the & before self, just as we did in &Rectangle. Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.

We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

`impl` blocks can also be used for associated functions. These dont take the `self` as a param, they're associated with the struct.

```
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

This can then be called with `Rectangle::square(3);`

You can have a single impl block or multiple.

## ENUMS

```
enum IpAddrKind {
    V4,
    V6,
}
```

`let four = IpAddrKind::V4;`

Like other languages, we can associate values with enum values as well.

```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

The types of data in the enums can be a variety of values, they dont need to be a single type.

Enums can also have methods defined using `impl`.

### Option Enum

Like `Optional` in Java. In Rust:

```
enum Option<T> {
    Some(T),
    None,
}
```

```
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```

^^ This will throw an error since you can't add an i8 and an `Option<i8>`
More on that `https://doc.rust-lang.org/std/option/enum.Option.html`
Useful methods: `is_some`, `is_none`, `contains`, `as_ref`, `as_mut`, `unwrap`, `unwrap_or`, `unwrap_or_else`, `map`

## MATCH AND IF LET

Matches are designed to match against the possibilities. Note that the match needs to be exhaustive so all possibilities need to be covered or defaults need to be provided.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

```
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

If you only care about a few possibilities, you can match for those and then put a placeholder to account for the rest.

```
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

If you only care about a single possibility, then it may be better to just use if let instead to avoid the verbosity.

```
if let Some(3) = some_u8_value {
    println!("three");
}
```
