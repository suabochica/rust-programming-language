# Chapter 2: Programming a Guessing Game
Let's jump into Rust by working through a hands-on project together. The idea is introduces a few common Rust concepts by showing you how to use them in a real program. You will learn:

+ `let` and `match` methods
+ Associated functions
+ Using external crates

Time to put in practice the fundamentals.

We will implement a guessing game:

The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## Seting Up a New Project

Let'ts run the next command to make a new project using Cargo:

```
$ cargo new guessing_game
$ cd guessing_game
```

As before, two files were created (`Cargo.toml` and `src/main.rs`) inside a `guessing_game` directory. By default, there should be a "Hello, world!" program, so let's compile the project with `cargo run`.

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Hello, world!
```

To implement the guessing game, we will modify the contents of the `src/main.rs` file.

## Processing a Guess
The first part of the guessiong game program will ask for user input, process that inpunt, and check that the input is the expected form. To start let's enter the next code in `src/main.rs`

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

This code contains a lot of information, so let's go over it line by line. To obtain user input and then print the result as output, we need to bring the `io` (input/output) library into scope. The `io` library comes from the standard library known as `std`:

```rust
use std::io;
```

By default, Rust brings only few types into the scope of every program in the prelude. If a type you want to use is not in the prelude, you have to bring that type into scope explicitly with a `use` statement. Using the `std:io` library provides you with a number of useful features, including the ability to accept user input.

Again, the `main()` function is the entry point into the program:

```rust
fn main() {}
```

the `fn`syntax declares a new function, the parentheses `()`, indicate there are no parameters, and the curly brackets `{}` group the body of the function.

As you learn in chapter 1, `println!` is a macro that prints a string to the screen:

```rust
    println!("Guess the number!");
    println!("Please input your guess.");
```

This code is printing a prompt stating what the game is and requesting input from the user.

### Storing values with Variables

### Links
+ [Module std::predule](https://doc.rust-lang.org/std/prelude/index.html)
