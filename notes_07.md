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

## LIFETIMES

Every reference in Rust has a lifetime: the scope for which that reference is valid. Lifetimes are usually implicit and inferred, just like types. If lifetimes of references could be related in some way we annotate this. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

Consider the following:

```
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
```

Here, we assign the reference of x to r but x doesn't live until the println! so the Rust borrow checker throws an error.

```
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
```

The above fixes that since the annotated lifetime for x is longer than r.

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//|
//| fn longest(x: &str, y: &str) -> &str {
//|                                 ^ expected lifetime parameter
//|
// = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

The problem here is that longest function doesnt know if the lifetime of x or y is being returned.

```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

Based on the above and the error message we got, we can alter the function:

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This will compile. When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called.

```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

If we modify the function like above the compilation will fail due to the borrow checker complaining that the borrowed value doesnt live long enough. The compiler knows the function will return something that has the same lifetime as arguments. In the main(), one of the arguments passed has a shorter lifetime so the borrow checker will want to make sure that value is only used as long as the shorter of the two lifetimes.

```
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter

```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

On the otherhand, the following won't compile because the return value's lifetime isn't related to the lifetime of the parameters. The result goes out of scope gets cleaned up at the end of the function and we trying to return a reference to result from the function:

```
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

This struct has one field, part, that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

There are certain patterns that Rust programmers noticed regarding lifetimes which the compiler checks for; so there are some cases where you dont have to define lifetimes every time. Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes. There are 3 rules the compiler uses:
i) each parameter that is a reference gets its own lifetime parameter
ii) if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32.`
iii) if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

`fn first_word(s: &str) -> &str {` here, as the compiler, we apply rule 1 and get the lifetime of the input, since theres only 1 input, the second rule applies on the output and we have all the lifetimes we need.

`fn longest(x: &str, y: &str) -> &str {` on the other hand, here we put 2 different lifetimes on the inputs, rule 2 isnt valid and neither is rule 3. Since we dont know the lifetime of the return we throw an error.

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

Here, rule 1 applies and both inputs get their lifetimes, and since one is self, we aply rule 3 and we have all the lifetimes we need.

## STATIC

One special lifetime 'static, which means that this reference can live for the entire duration of the program.

See https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#generic-type-parameters-trait-bounds-and-lifetimes-together
