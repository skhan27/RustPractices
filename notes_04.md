# NOTES: PART 4 -> Packages, Crates, Modules, Paths

## Packages and Crates

A crate is a binary or library. THe crate root is a source file that the Rust compiler starts from and makes up the root module of the crate.

A package is one or more crates that provide a set of functionality. The package contains a Cargo.toml file that describes how the crates are built.

When you use `cargo new <project_name>` Cargo creates a Cargo.toml file which means we got a package. Theres no src/main.rs in the toml file but its because Cargo has a convention that the src/main.rs is the crate root of a binary crate with the same name as the package. Similarly, if the package directory contains src/lib.rs the package contains a library crate with the same name as the package and the src/lib.rs is the crate root.

If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate. A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.

Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the rand crate and prevents potential conflicts. For example, the rand crate provides a trait named Rng. We can also define a struct named Rng in our own crate. Because a crate’s functionality is namespaced in its own scope, when we add rand as a dependency, the compiler isn’t confused about what the name Rng refers to. In our crate, it refers to the struct Rng that we defined. We would access the Rng trait from the rand crate as rand::Rng.

## Modules

Modules organize code within a crate for readability and reuse. Modules also control privacy of items.

> EXAMPLE:
> In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house. Front of house is where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize the functions into nested modules. Create a new library named restaurant by running `cargo new --lib restaurant`
Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions:

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

### Paths in the module tree

A path can take two forms:

An absolute path starts from a crate root by using a crate name or a literal crate.
A relative path starts from the current module and uses self, super, or an identifier in the current module.

The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default. Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. The reason is that child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. We use `pub` to make things available.

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

We can use `super` as well to refer to an ancestor from a child.

```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

We can also use pub to designate structs and enums as public, but there are a few extra details. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword

### `use` Keyword for paths

Instead of having super verbose long paths we can use `use` to simplify the path and bring items into scope:

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

It is idiomatic to bring the parents module into scope and then use the function rather than using `use` all the way until the function (in the above it would mean adding the function to the `use` line and then just calling the function instead of doing `hosting::` first). This is because that prevents confusion regarding where the function actually lives.

When it comes to enums, structs and other items though, it is idiomatic to specify the full path instead.

`use` can be combined with `as` to rename the import. ` use std::io::Result as IoResult;`

Can also combine `pub` and `use`.

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

To use external packages from Crate.io we need to add them to the toml file under the dependencies and then `use` them.

Quick example of modules in files:
https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
