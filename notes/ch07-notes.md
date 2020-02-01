# Chapter 07: Managing Growing Projects with Packages, Crates, and Modules.

As you write large programs, organizing your code will be important because keeping track of your entire program in your head will become impossible. By grouping related functionality and separating code with distinct features, you will clarify where to find code that implements a particular feature and where to go to change how a feature works.

The programs we have written so far have been one module in one file. As a project grows, you can organize code by splitting it into multiple modules and then multiple file. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. This chapter covers all these techniques. For very large projects of a set of interrelated package that evolve together, Cargo provides workspaces, which we will cover in chapter 14.

In addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level. Once you have implemented an operation, other code can call that code via the code's public interface without knowing how the implementation works. The way you write code define which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head.

A related concept is **scope**, the nested context in which code is written has a set of names that are defines as _in scope_. When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spots refers to a variable, function, struct, enum, module, constant, or other item and what item means. You can create scopes and change which names are in or out of scope. You cannot have two items with the same name in the same scope, tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code's organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the _module system_ and include.

+ **Packages**: A Cargo feature that lets you build, test, and share crates.
+ **Crates**: A tree of modules that produces a library or executable.
+ **Modules and use**: Let your control the organization, scope, and privacy paths.
+ **Paths**: A way of naming an item, such as a struct, function, or module

In this chapter, we will cover all these features, discuss how they interact, and explain how to use them to manage scope. By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro.

## Index
1. Packages and Crates
2. Defining Modules to Control Scope and Privacy
3. Paths to Referring to an Item in the Module Tree
4. Bring Paths Into Scope with the `use` Keyword
5. Separating Modules into Different Files.

## 1. Packages and Crates
The first parts of the module system we will cover are package and crates. A crate is a binary or a library. The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate. A _package_ is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

Several rules determine what a package can contain. A package must contain zero or one library crates, and no more. It can contain as many binary crates as you would like, but it must contain at least one crate.

Let's walk through what happens when we create a package. First, we enter the command `cargo new`:

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

When we entered the command, Cargo created a `Cargo.toml` file, giving us a package. Looking at the contents of `Cargo.toml` there is no mention of `src/main.rs` because Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root. cargo passes the crate root files to `rustc` to build the library or binary.

Here we have a package that only contains `src/main.rs` meaning it only contains a binary crate named `my-project`. If a package contains `src/main.rs` and `src/lib.rs`, it has two crates. A library and a binary, both with the same name as the package. A package can have multiple binary crates placing in the `src/bin` directory, each file will be a separate binary crate.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects. For example, the `rand` crate used in the guessing number game, provides functionality that generates random numbers. We can use that functionality in our projects by bringing the `rand crate into our project's scope. All the functionality provided by the `rand` crate is accessible through a crate's name, `rand`.

Keeping a crate's functionality in its own scope clarifies whether particular functionality is defined in our crate or the `rand` crate and prevents potential conflicts. For example, the `rand` crate provides a trait name `Rng`. we can also define `struct` named `Rng` in our crate. Because a crate's functionality is namespaced in its own scope, when we add `rand` as a dependency, the compiler is not confused about what the name `Rng` refers to. In our crate, it refers to the `struct Rng` that we defined. We would access the `Rng` trait from the `rand` crate as `rand::Rng`.

## 2. Defining Modules to Control Scope and Privacy
_Modules_ let us organize code within a crate into groups for readability and easy reuse. Modules also control the _privacy_ items, which is whether an item can be used by outside code (public) or is internal implementation detail and not available for outside use (private).

As an example, let's write a library crate that provides the functionality of a restaurant. We will define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than actually implement a restaurant code.

In the restaurant industry, some parts of a restaurant are referred to as _front of house_ and others a_back of house_. Front of house is where customers are, this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of the house is where the chefs and cooks work in the kitchen, dishwasher clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize the function into nested modules. Create a new library named `restaurant` by running `cargo new --lib restaurant`, then put the code in `src/lib.rs` to define some modules and function signatures.

```rust
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

We define a module by starting with the `mod` keyword and then specify the name of the module (`front_of_house`) and place curly bracket around the body of the module. Inside modules, we can have other modules, as in this case with `hosting` and `seving`. Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions.

By using modules, we can group related definitions together and name why they are related. Programmers, using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather that having to read through all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that `src/main.rs` and `src/lib.rs` are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at root of the crate's module structure, known as the **module tree**.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

This tree shows how some of the modules nest inside one another. The three also shows that some modules are _siblings_ to each other, meaning they are defined in the same module. To continue with the family metaphor, if module A is contained inside module B, we say that module A is the child of module B and the module B is the _parent_ of module A. Notice that the entire module tree is rooted under the implicit module named `crate`.

This module tree might remind you of the file system's directory tree on your computer. This is a very apt comparison. Just like directories in a file system, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

## 3. Paths to Referring to an Item in the Module Tree
To show Rust where to find an item in module tree, we use a path in the same way we use a path when navigating a file system. If we want to call a function, we need to know its path.

A path can take two forms:

+ An _absolute path_ starts from a crate root by using a crate name or a literal `crate`.
+ A _relative path_ starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifier separated by double colons `::`.

Let's return the module tree that we create for the restaurant concepts. How we do call the `add_to_waitlist` function? The answer is the path of the `add_to_waitlist` function. We will simplified our restaurant library code removing some of the modules and functions. We will show two ways to call the `add_to_waitlist` function from the new function `eat_at_restaurant` defined in the crate root. The `eat_at_restaurant` function is part of our crate's public API, so we mark it the `pub` keyword. Keep in mind that this example won't compile just yet, we will explain why in a bit.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The first time we call `add_to_waitlist` function in `eat_at_restaurant`, we use an absolute path. The `add_to_waitlist` is defined in the same crate as `eat_at_restaurant`, which means we can use the `crate` keyword to start an absolute path.

After `crate`, we include each of the successive modules until we make our way to `add_to_waitlist`. You cam imagine a file system with the same structure, and we would specify the path `/front_of_house/hosting/add_to_waitlist` to run the `add_to_waitlist` program. Using the `crate` name to start from the crate root is like using `/` to start from the file system root in the shell.

The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a relative path. The path starts with `front_of_house`, the name of the module defined at the same leave of the module tree as `eat_at_restaurant`. Here the file system equivalent would be using the path `front_of_house/hosting/add_to_waitlist`. Starting with a name means that the path is relative.

Choosing whether to use a relative or absolute path is a decision you will make based on your project. The decision should depend on whether you are more likely to move item definition code separately from or together with the code that uses the item. For example, if we move the `front_of_house` module and the `eat_to_restaurant` function into a module called _customer_experience, we would need to update the absolute path to `add_to_waitlist`, but the relative path would still be valid. However, If we moved the `eat_at_restaurant` function separately into a module named _dining_, the absolute path to the `add_to_waitlist` could stay the same, but the relative path would need to be updated. Our preference is to specify absolute paths because it is more likely to move code definitions and item call independently of each other.

If we try to compile our last code we will get:

```
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^

```

The error messages say that module `hosting` is private. In other words, we have the correct paths for the `hosting` module and the `add_to_waitlist` function, but Rust won't let us use them because it does not have access to the private sections.

Modules are not useful only for organizing your code, They also define Rust's **privacy boundary**. The line that encapsulates the implementation details external code is not allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.

The way privacy works in Rust is that all items are private by default. Items in a parent module cannot use the private items inside the child modules, but items in child modules can use the item in their ancestor modules. The reason is that child modules can see the context in which they are defined. To continue with the restaurant metaphor, think of the privacy rules as being like the back office of a restaurant. what foes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant in which they operate.

Rust chose to have the module system function this way so that hiding inner implementation details is the default. That way, you know which parts of the inner code you can change without braking outer code. But you can expose inner parts of child modules to outer ancestor modules by using the `pub` keyword to make a item public.

### Exposing Path with the `pub` Keywords
Let's return to our compiler error that told use that the `hosting` module is private. We want that the `eat_at_restaurant` function in the parent module to have access to the `add_to_waitlist` function in the child module. So let's add the `pub` keyword before our `hosting module`. However, This change is not enough, because the contents of `hosting` are still private. Making the modules public does not make its contents public. The `pub` keyword on a module only lets code in its ancestor modules refer to it. That means that we have to add `pub` keyword in the `add_to_waitlist` function. Now the code will compile.

Let's look at the absolute and the relative path and double check why adding the `pub` keyword lets us use the paths in `add_to_waitlist` with respect to the privacy rules.

In the absolute path, we start with `crate`, the root of our crate's module tree. Then the `font_of_house` module is defined in the crate root. The `font_of_house` module is no public, but because the `eat_at_restaurant` function is defined in the same module as `front_of_house`, we can refer to `front_of_house` from `eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can access the parent module of `hosting`, so we can access `hosting`. Finally, the `add_to_waitlist` function is marked with `pub` and we can access its parent module, so this function call works.

In the relative path, the logic is the same as the absolute path except for the first step. Rather than starting from the crate root, the path starts from `front_of_house`. The `front_of_house` module is defined within the same modules as `eat_at_restaurant`, so the relative path starting from the module in which `eat_at_restaurant`, so the relative path starting from the module in which `eat_at_restaurant` is defined works. Then, because `hosting` and `add_to_waitlist` are marked with `pub`, the rest of the path works, and this function call is valid.

### Starting Relative Paths with `super`
We can also construct relative paths that begin in the parent module by using `super` at the start of the path. This is like starting a file system path with the `..` syntax. Why would we want to do this?

Consider the next code that models the situation in which a chef fixes and incorrect order and personally brings it out to the customer. The function `fix_incorrect_order` calls the function `server_order` by specifying the path to `server_order` starting `super`.

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn main() {}
```

The `fix_incorrect_order` function in the `back_of_house` module, so we can use `super` to go to the parent module of `back_of_house`, which in this case is `crate`, the root. From there, we look for `server_order` and find it. Success! We think the `back_of_house` module and the `serve_order` function are likely to stay in the same relationship to each other and get moved together should we decide to recognize the crate's module tree. Therefore, we used `super` so we will have fewer places to update code in the future if this code gets moved to a different module.

### Making Structs and Enums Public
We can also use `pub` to designate structs and enums as public, but there are a few extra details. If we use `pub` before a struct definition, we make the struct public, but the struct's fields will still be private. We can make each field public or not on a case-by-case basis. The next snippet defined a public `back_of_house::Breakfast` struct with a public `toast` field but a private `seasonal_fruit` field. This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal based on what is in season and in stock. The available fruit changes quickly, so customers cannot choose the fruit or even see which fruit they will get.

```rust

#![allow(unused_variables)]
fn main() {
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
}
```

Because the toast field in the `back_of_house::Breakfast` struct is public, in `eat_at_restaurant` we can write and read to the `toast` field using dot notation. Notice that we cannot use the `seasonal_fruit` field in `eat_at_restaurant` because `seasonal_fruit` is private. Try uncommenting the line modifying the `seasonal_fruit` field values to see what error you get.

Also, note that because `back_of_house::Breakfast` has a private field, the struct needs to provide a public associated function that constructs an instance of `Breakfast`. If `Breakfast` did not have such a function, we could not create an instance of `Breakfast` in `eat_at_restaurant` because we could not set the value of the private `seasonal_fruit` field at `eat_at_restaurant`.

In contrast, if we make an enum public, all of its variants are the public. We only need the `pub` before the `enum` keyword, as shown below.

```rust

#![allow(unused_variables)]
fn main() {
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() { 
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
}

```

Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad` variants in `eat_at_restaurant`. Enums are not very useful unless their variants are public. It would be annoying to have to annotate all enum variants with `pub` in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with `pub`.

There is one more situation involving `pub` that we have not covered, and that is our last module system feature. The `use` keyword. We will cover use by itself first, and then we will show how to combine `pub` and `use`.

## 4. Bring Paths Into Scope with the `use` Keyword
It might seem like the paths we have written to call functions so far are inconveniently long and repetitive. Fortunately, there is a way to simplify this process. We can bring a path into a scope once and then call the items in that path as if they are local items with the `use` keyword.

The next snippet brings the `crate::front_of_house::hosting` module into the scope of the `eat_at_restaurant` function so we only have to specify `hosting::add_to_waitlist` to call the `add_to_waitlist` function in `eat_at_restaurant`.

```rust
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
fn main() {}
```

Adding `use` and a path in scope is similar to creating a symbolic link in the file system. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope, just as though the `hosting` module had been defined in the crate root. Paths brought into scope with `use` also check privacy, like any other paths.

You can also bring an item into scope with `use` and a relative path. The equivalent code with relative path is shown below:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}
```

### Creating Idiomatic `use` Paths

In the last snippet, you might wondered why we specified `use crate::front_of_house` and then called `hosting::add_to_waitlist` in the `eat_at_restaurant` rather than specifying the `use` path all the way out to the `add_to_waitlist` function to achieve the same result.

Both scenarios accomplish the same task, but put the code until `use crate::front_of_house` is the idiomatic way to bring a function into scope with `use`. Bringing the function's parent module when calling the function makes it clear that the function is not locally defined while still minimizing repetition of the full path. The second case makes that the code was unclear as to there `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`, it is idiomatic to specify the full path. Below we shows the idiomatic way to bring the standards library's `HashMap` struct into the scope of a binary crate.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

There is no strong reason behind this idiom. It is just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

The exception to this idiom is if we are bringing two items with the same name into scope with `use` statement, because Rust does not allow that. Code below shows how to bring two `Result` types into scope that have the same name but different parent modules and how to refer to them.

```rust

#![allow(unused_variables)]
fn main() {
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
}
```

As you can see, using the parent modules distinguishes the two `Result` types. If instead we specified `use std::fmt::Result` and `use std::io::Result` we would have two `Result` types in the same scope and Roust would not know which one we meant when we used `Result`.

### Providing New Names with the `as` Keyword
There is another solution to the problem of bringing two types of the same name into the same scope with `use`. After the path, we can specify `as` and a new local name, or alias, for the type. The next snippet shows another way to write the last code by renaming one of the `Result` types using `as`.

```rust

#![allow(unused_variables)]
fn main() {
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
}
```

In the second `use` statement, we chose the new name `IoResult` dor the `std::io::Result` type, which won't conflict with the `Result` from `std::fmt` that we have also brought into scope. Both samples are considered idiomatic, so the choice is up to you.

### Re-exporting Names with `pub use`

### Using External Packages

### Using Nested Paths to Clean Up Large `use` Lists

### The Glob Operator



## 5. Separating Modules into Different Files.

