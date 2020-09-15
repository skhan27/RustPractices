# Notes: Part 6: Errors

Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array. Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

## Panic!

When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. This most commonly occurs when a bug of some kind has been detected and it’s not clear to the programmer how to handle the error.

When a panic occurs, Rust walks back up the stack and cleans up data from each function. This is a lot of work so if the binary needs to be as small as possible, the alternative is to abort on panic and clean up using the OS. To do so, add `panic = 'abort'` to the `[profile]` section in the toml file.

Panic can be manually called simply: `panic!("crash and burn");`

## Result<T, E>

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

Since we have to handle the Result for each case in the above, the code gets a bit verbose. We can use `unwrap` or `expect` methods depending on our intent if we only want one thing to happen. With `unwrap`, if there is an Err, it will call the panic! macro. We use `expect` in the same way as `unwrap`: to return the file handle or call the panic! macro. The error message used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic! message that unwrap uses.

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // OR
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

We can also propagate errors upwards from a function to allow the function that called it to handle the error.

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

The above is verbose and can be shortened:

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code. We can chain this to make it even simpler:

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

Note that the main function is special, and there are restrictions on what its return type must be. One valid return type for main is (), and conveniently, another valid return type is Result<T, E>

```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```
