# Notes: Part 2

## Ownership

### Stack vs Heap

All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. Because the pointer is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

### Ownership rules

-   Each value in Rust has a variable that’s called its owner.
-   There can only be one owner at a time.
-   When the owner goes out of scope, the value will be dropped.

```
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

The `String` type in Rust is more complex than string literals. It can be mutated and has a lot more functionality like `from`, `push_str` etc methods.

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

The memory must be requested from the memory allocator at runtime.
We need a way of returning this memory to the allocator when we’re done with our String.
That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track and cleans up memory that isn’t being used anymore, and we don’t need to think about it. Without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. When `String` goes out of scope, Rust calls the `drop` function to return the memory.

### Moving Data

When looking at simple integers, `let x = 5` pushes the value of 5 on the stack. `let y = x` will then push the value of 5 on the stack for y. On the other hand, Strings behave differently.

A String has data stored on the stack and data on the heap. A pointer to the contents on the heap, the length and capacity are stored on the stack. The contents, as mentioned are on the heap.

```
let s1 = String::from("hello");
let s2 = s1;
```

In the above, the values on the stack are copied, which means the pointer is copied but the data in the heap is still the same so `s1` and `s2` point to the same data. So far this is much like other languages. However, if this was all we would run into a memory safety bug when both `s1` and `s2` go out of scope and try to free the same memory. We don't want to free memory twice. To prevent this, Rust does one more thing: it considers s1 no longer valid and therefore doesn't need to free anything when `s1` goes out of scope. If you try to use `s1` after `s2` is created you will run into an error because Rust has invalidated the reference. This whole process is considered `moving` as opposed to a shallow copy. The other implication here is that Rust will never create "deep" copies and so any automatic copying can be assumed to be inexpensive.

If we do want to deeply copy the string data, we can use the `clone` method. This is not inexpensive since data on the heap is copied.

Scalar types can be `Copy` in that they have that trait and are stored on the stack.

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Rust also supports returning multiple values from functions using Tuples.

## References and Borrowing

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

`&` are references. They allow you to refer to some value without taking ownership.
`*` is the dereference operator, discussed later.
In the calculate_length method we dont own s since we got a reference. We are \_borrowing\* it.When you have references as parameters of a function, you dont have to return the values to give back ownership since you never had ownership. Trying to modify a borrowed value will not work. Like variables, references are immutable by default. However, we can change it to a mutable reference:

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

In this case, `s` had to be `mut` and then the reference was as well.
There is a restriction here though: only one mutable reference can be made in a particular scope.

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

The above will fail due to a compiler error. This way we prevent a data race.
We can use `{}` to create a new scope, allowing multiple references just not simultaneous ones.

You can not have a mutable reference while there is also an immutable reference either.

```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

Remember however that the scope of a reference is from where its introduced to the last time its used. So making a small adjustment to the above will work:

```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

Rust also prevents dangling pointers with a compile time error.

```
fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

The solution here is to return the `String` itself not the reference.

## SLICE TYPE

A `String` slice is a reference to a part of the String.

```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

We can create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice. If your starting index is 0 you can skip it. Same if your ending index is the last elements index. String slices have the type `&str`

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

String literals are `&str` types and are immutable because `&str` are immutable references.

Slices can also exist for other types apart from just String.

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```
