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
