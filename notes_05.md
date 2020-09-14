# Notes: Part 5 -> Collections

## VECTORS

Vectors store more than one value, single data structure and puts all values next to each other in memory. Only values of the same type.
`let v: Vec<i32> = Vec::new();`
Rust also provides a macro for creating a new vector that holds the values you give it `let v = vec![1, 2, 3];`. In this case the type would be vec<i32> since thats the default integer type.
To add values we can use the `push` method but since we are changing the values, we need to make it mutable as well:

```
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

When a vector goes out of scope, it is dropped and all of its contents are as well.

Getting values from a vector is done in 2 ways: indexing syntax or the `get` method. Note the types in the following:

```
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
```

When you use the indexing syntax you get a reference and if you use the `get` method you get an `Option<T>`
This means that if you refer to an element that doesn't exist i.e. length is 2 and you try to get the 5th element, the indexing syntax will cause a panic while the get simply returns a None (remember Option<T> has Some<T> and None).

if the reference is valid, the borrow checker in rust enforces the ownership and borrowing rules.

```
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
```

The above will fail because there is an immutable borrow happening with the indexing syntax and then a mutable borrow when we `push`
This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, ifthere isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

We can iterate over the values of a vector if we just want to read them:

```
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
```

if we want to make changes to the elements, we can make things mutable:

```
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
```

Use an enum to store multiple types of data in a vector

```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

## Strings

` let mut s = String::new();`
`let s = String::from("initial contents");`

```
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
```

use push_str to append string slice:

```
 let mut s = String::from("foo");
s.push_str("bar");
```

```
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

```
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
```

Indexing into strings is not valid in Rust. String is a wrapper over Vec<u8>. use ranges to create string slices with caution, because doing so can crash your program.
Read the in depth explanation in the book: https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings

## Hash Maps

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

Just like vectors, hash maps store their data on the heap.
For example, if we had the team names and initial scores in two separate vectors, we could use the zip method to create a vector of tuples where “Blue” is paired with 10, and so forth. Then we could use the collect method to turn that vector of tuples into a hash map

```
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
```

For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values

```
use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
```

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

`get` returns an Option<T> so value will be Some<T> or None

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

Values can be overwritten:

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
```

It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it. Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist. The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

We can also update values based on old values by using the same API:

```
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```
