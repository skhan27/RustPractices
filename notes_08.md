# NOTES: part 8 -> Automated Testing

```
#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
       assert_eq!(2 + 2, 4);
   }
}
```

Run tests using `cargo test`

#[test] before fn tells the test runner that this function is a test. Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

assert_eq! and assert_ne! macros compare two arguments for equality or inequality, respectively.

If we want to test and expect a panic we can add #[should_panic] under #[test]
Tests that use should_panic can be imprecise because they only indicate that the code has caused some panic. A should_panic test would pass even if the test panics for a different reason from the one we were expecting to happen. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text.

```
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

We can also write tests that use Result<T, E>

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Configuring Tests

Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator --.
`cargo test -- --test-threads=1` tells the runner to not use any parallelism. Increasing this will tell the runner to run tests in parallel

`cargo test -- --show-output` is needed if you want to see any println! used in your functions.

We can pass the name of any test function to cargo test to run only that test

We can also specify tests that match a certain subset of tests by matching certain names. `cargo test add` will run any tests that have `add` in their name.

```
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

the ignore line excludes the test when you just run `cargo test`. You can run ignored tests using `cargo test -- --ignored`

Organization: https://doc.rust-lang.org/book/ch11-03-test-organization.html
