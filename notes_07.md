# Notes: Part 7 -> Generic Types, Traits, Lifetimes

## GENERICS

Rusts generics are performant due to monomorphization. Rust turns generic code into specific code when compiled. The compiler reads the values that have been used as the generic types and expands the generic definition into definitions with those types. Therefore there is no runtime cost associated with them.

```
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## TRAITS

A trait tells the compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior. Traits are similar to interfaces in other languages but with some differences.
A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

```

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

We can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are defined in the standard library and aren’t local to our aggregator crate. This restriction is part of a property of programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

We can add a default implementations to the methods in a trait that the types implementing those traits then dont need to implement.

```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

The syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation. Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.

We can pass Traits as parameters to functions. This allows us to limit our function to types that implement that trait

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

The above is syntactic sugar on the more verbose version:

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

`pub fn notify(item1: &impl Summary, item2: &impl Summary) {`

`pub fn notify<T: Summary>(item1: &T, item2: &T) {`

Based on above, we can also specify multiple trait bounds using `+`
`pub fn notify(item: &(impl Summary + Display)) {`
`pub fn notify<T: Summary + Display>(item: &T) {`
This can get really cluttered when writing functions with many types and traits so we can use the `where` clause and turn
`fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {`
into

```
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

We can also specify the return types to those that implement traits but this is limited to a single type i.e a function can only return a single type that implements that trait.

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

SEE: https://doc.rust-lang.org/book/ch10-02-traits.html#fixing-the-largest-function-with-trait-bounds
summary: Just adding `PartialOrd` to the bound of the type will not work because a generic list will include items that are those that live on the heap and those that live on the stack. We need to limit the types to those that are on the stack and those types implement `Copy` trait.

```
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

Lastly, we can also conditionally implement methods for types that implement certain traits. Below every Pair implements the `new` method but only Pairs of `T` where the type of `T` implements the Display and PartialOrd traits have the `cmp_display` method implemented.

```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
