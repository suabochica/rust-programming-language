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

#e Processing a Guess
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
New we will create a place to store the user input, like this:

```rs
let mut guess = String::new();
```

Now the program is getting interesting. There is a lot going on this little line. Notice that this is a let statement, which is used to create a _variable_. Here is another example:

```rs
let foo = bar;
```

This line creates a new variable named `foo` and binds it to the value of the `bar` variable. In Rust, variables are immutable by default. We will discussing this concept in detail in the [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability) section in Chapter 3. The following example shows how to use `mut` before the variable name to make a variable mutable.

```rs
let foo = 5; // immutable
let mut bar = 5; // mutable
```

> Note: The // syntax starts a comment that continues until the end of the line. Rust ignores everything in comments, which are discussed in more detail in Chapter 3.

Let's return the guessing game program. You now know that `let mut guess` will introduce a mutable variable named `guess`. On the other side of the equal sign is the value `String::new`, that is a function that returns a new instance of a `String`. `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The `::` syntax in the `::new` line indicates that `new` is an _associated function_ of the `String` type. An associated function is implemented on a type, in this case `String`, rather than on a particular instance of `String`. Some languages call this a _static method_.

This `new` function creates a new, empty string. You will find a `new` function on many types, because it is a common name for a function that makes a new values of some kind.

To summarize, the `let mut guess = String::new` line has created a mutable variable that is currently bound to a new, empty instance of a `String`.

Recall that we included the i/o functionality from the standard library with `use std::io` on the first line of the program. Now we will call the `stdin` function from the `io` module:

```rs
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

If we had not put the `use std::io` line at the beginning of the program, we could have written this function call ass `std::io::stdin`. The `stdin` function return an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.

The next part of the code ,`.read_line(&mut guess)`, calls the `read_line` method on the standard input handle to get input from the user. We are also passing one argument to `read_line`: `&mut guess`.

The job of `read_line` is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string's content by adding the user input.

The ampersand `&` indicates that this argument is a __reference_, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust's major advantages is how safe and easy it is to use references. You do not need to know a lot of those details to finish the program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write `&mut guess` rather than `&guess` to make it mutable. In chapter four we will explain references more thoroughly.

### Handling Potential Failure with the `Result` Type.

### Links
+ [Module std::predule](https://doc.rust-lang.org/std/prelude/index.html)
+ [String type](https://doc.rust-lang.org/std/string/struct.String.html)
+ [std::io::Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)
