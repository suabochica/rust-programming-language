# Chapter 2: Programming a Guessing Game

## Index
1. Setting Up a New Project
2. Processing a Guess
3. Generating a Secret Number
4. Comparing the Guess to the Secret Number
5. Allowing Multiple Guesses with Looping



Let's jump into Rust by working through a hands-on project together. The idea is introduces a few common Rust concepts by showing you how to use them in a real program. You will learn:

+ `let` and `match` methods
+ Associated functions
+ Using external crates

Time to put in practice the fundamentals.

We will implement a guessing game:

The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## 1. Setting Up a New Project

Let's run the next command to make a new project using Cargo:

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

## 2. Processing a Guess
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

```rust
let mut guess = String::new();
```

Now the program is getting interesting. There is a lot going on this little line. Notice that this is a let statement, which is used to create a _variable_. Here is another example:

```rust
let foo = bar;
```

This line creates a new variable named `foo` and binds it to the value of the `bar` variable. In Rust, variables are immutable by default. We will discussing this concept in detail in the [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability) section in Chapter 3. The following example shows how to use `mut` before the variable name to make a variable mutable.

```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```

> Note: The // syntax starts a comment that continues until the end of the line. Rust ignores everything in comments, which are discussed in more detail in Chapter 3.

Let's return the guessing game program. You now know that `let mut guess` will introduce a mutable variable named `guess`. On the other side of the equal sign is the value `String::new`, that is a function that returns a new instance of a `String`. `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The `::` syntax in the `::new` line indicates that `new` is an _associated function_ of the `String` type. An associated function is implemented on a type, in this case `String`, rather than on a particular instance of `String`. Some languages call this a _static method_.

This `new` function creates a new, empty string. You will find a `new` function on many types, because it is a common name for a function that makes a new values of some kind.

To summarize, the `let mut guess = String::new` line has created a mutable variable that is currently bound to a new, empty instance of a `String`.

Recall that we included the i/o functionality from the standard library with `use std::io` on the first line of the program. Now we will call the `stdin` function from the `io` module:

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

If we had not put the `use std::io` line at the beginning of the program, we could have written this function call ass `std::io::stdin`. The `stdin` function return an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.

The next part of the code ,`.read_line(&mut guess)`, calls the `read_line` method on the standard input handle to get input from the user. We are also passing one argument to `read_line`: `&mut guess`.

The job of `read_line` is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string's content by adding the user input.

The ampersand `&` indicates that this argument is a __reference_, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust's major advantages is how safe and easy it is to use references. You do not need to know a lot of those details to finish the program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write `&mut guess` rather than `&guess` to make it mutable. In chapter four we will explain references more thoroughly.

### Handling Potential Failure with the `Result` Type.
The second part of the method is:

```rust
.expect("Failed to read line")
```

When you call a method with the `.foo()` syntax, it is often wise to introduce a new line and other white space to help break up long lines, because one long line is difficult to read. So, it is best to divide it: two lines for two method for two lines for two method calls. Now let's discuss what this lines does.

As mentioned earlier, `read_line` puts what the user types into the string we are passing it, but it also return a value – in this case, and `io::Result` –. Rust has a number of types named `Result` in its standard library: a generic `Result` as well as specific versions for sub modules, such as `io::Result`.

The `Result` type are _enumerations_ (a.k.a _enums_). An enumeration is a type that can have a fixed set of values, and those values are called the _enum's variants_. Chapter 6 will cover enums in more detail.

For `Result`, the variants are `Ok` or `Err`. The `Ok` variant indicates the operations was successful, and inside `Ok` is the successfully generated values. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

The purpose of these `Result` type is to encode error-handling information. Values of the `Result` type, like values of any type, have methods defined on them. An instance of `io::Result` has an `expect` method that you can call. If this instance of `io::Result` is an `Err` value, expect will cause the program to crash and display the message that you passed as an argument to `expect`. If the `read-line` methods returns an `Err`, it would likely be the result of an error coming from the underlying operating system. If this instance of the `io::Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that values to you so you can use it. In this case, that values is the number of bytes in what the user entered into standard input.

If you do not call `expect`, the program will compile, but you will get a warning:

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

Rust warns that you have not used the `Result` value returned from `read_line`, indicating that the program has not handled a possible error.

the right way to suppress the warning is to actually write error handling, but because you want to crash this program when a problem occurs, you van use `expect`. You will learn about recovering from errors in Chapter 9.

### Printing Values with `println!` Placeholders
Aside from the closing curly brackets, there is only one more line to discuss in the code added so far, which is the following:

```rust
println!("You guessed: {}", guess);
```

This line print the string we saved the user's input in. The set of curly brackets `{}`, is a placeholder. Think of `{}` as a little crab pincers that hold a value in place. You can print more than one value using curly brackets. The first set of curly brackets holds the first values listed after format string, the second set holds the second values, and so on. Printing multiple values in one call to `println!` would look like this:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y); // Prints x = 5 and y = 10.
```

### Testing First Part
Let is test the first part of the guessing game Run it using `cargo run`:

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

At this point, the first part of the game is done: we are getting input form the keyboard an then printing it.

### Links
+ [Module std::predule](https://doc.rust-lang.org/std/prelude/index.html)
+ [String type](https://doc.rust-lang.org/std/string/struct.String.html)
+ [std::io::Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)
+ [Result type](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type)
+ [println!](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#printing-values-with-println-placeholders)

## 3 Generating a Secret Number
Next, we need to generate a secret number that the user will try to guess. The secret number should be different every time so the is fun to play more than once. Let's use a random number functionality in its standard library. However, the Rust team does provide a `rand` crate.

### Using a Crate to Get More functionality
Remember that a crate is a collection of Rust source code files. The project we have been building is a _binary crate_, with is an executable. The `rand` crate is a _library crate_, which contains code intended to be used in other programs.

Cargo's use of external crates is where it really shines. Before we can write code that uses `rand`, we need to modify the _Cargo.toml_ file to include the `rand` crate as a dependency. Open that file an add the `rand` crate in the `[dependencies]` section.

```toml
[dependencies]
rand = "0.5.5"
```

In the _Cargo.toml_ file, everything that follows a header is part of a section that continues until another section starts. The `[dependencies]` section is where you tell Cargo which external crates your project depends on and which versions of those crates you require. In this case we will specify `rand` crate with the semantic version specifier `0.5.5`. Cargo understand **Semantic Versioning** (a.k.a SemVer), which is a standard for writing version numbers. The number `0.5.5` is a shorthand for `^0.5.5`, which means "any version that has a public API compatible with version 0.5.5".

Now, without changing any of the code, let's build the project:

```
$ cargo build
    Updating crates.io index
  Downloaded rand v0.5.5
  Downloaded libc v0.2.62
  Downloaded rand_core v0.2.2
  Downloaded rand_core v0.3.1
  Downloaded rand_core v0.4.2
   Compiling rand_core v0.4.2
   Compiling libc v0.2.62
   Compiling rand_core v0.3.1
   Compiling rand_core v0.2.2
   Compiling rand v0.5.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 s
```

You may see different version numbers, but they will be a compatible with the code, thanks to SemVer, and the lines may be in different order.

Now that we have an external dependency, Cargo fetches the latest version of everything from the _registry_, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for other to use.

After updating the registry, Cargo check `[dependencies]` section and downloads any crates you do not have yet. In this case, although we only listed `rand` as a dependency, Cargo also grabbed `libc` and `read_core`, because `rand` depends on those to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you won't get any output aside from the `Finished` line. Cargo knows it has already downloaded and compiled the dependencies, an you have not changed anything about them in your _Cargo.toml_ file. Cargo also knows that you have not changed anything about your code, so it does not recompile that either. With nothing to do, it simply exits.

If you open _src/main.rs_ file, make a trivial change, and then save it and build again, you will only see two lines of output:

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

These lines show Cargo only updates the build with your tiny change to the _src/main.rs_. Your dependencies have not changed, so Cargo knows it can reuse what it has already downloaded and compiled for those. It just rebuilds your part of the code.

#### Ensuring Rproducible Builds with _Cargo.lock File
Cargo has a mechanism that ensures you can rebuild the same artifact every time you builds code : Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, what happens if next week version `0.5.6` of the `rand` crate comes out and contains and important bug fix but also contains a regression that will break your code?

The answer to this problem is the _Cargo.lock_ file, which was created the first time you ran `cargo build` and is now in your _guessing_game_ directory. When you build a project for the first time, Cargo figures out all the version of the dependencies that fit the criteria and then writes them to the _Cargo.lock_ file. When you build your project in the future, Cargo will see thaht the _Cargo.lock_ file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at `0.5.5` until you explicitly upgrade, thank to the _Cargo.lock_ file.

#### Updating a Crate to Get a New Version
When you do want to update a crate, Cargo provides another command `update`, which will ignore the _Cargo.lock_ file and figure out all the latest versions that fit your specifications in _Cargo.toml_. If that works, Cargo will write those versions to the _Cargo.lock_ file.

But by default, Cargo will only look for versions greater than `0.5.5` and less than `0.6.0`. If the `rand` crate has released two versions, `0.5.5` and `0.6.0`, you would see the following if you ran `cargo update`.

```
$ cargo update
    Updating crates.io index
    Updating rand v0.5.5 -> v0.5.6
```

At this point, you would also notice a change in your _Cargo.lock_ file nothing that the version of the `rand` crate you are now using is `0.5.6`.

If you wanted to use `rand` version `0.6.0`, you would have to update the _Cargo.toml_ file to look like this instad:

```toml
[dependencies]
rand = "0.6.0"
```

The next time you run `cargo build`, Cargo will updated the registry of crates available and reevaluate your `rand` requirements according to the new version you have specified.

There is a lot of more to say about Cargo and its ecosystem which we will discuss in chapter 14, but for now, that is all you need to know. Cargo makes it very easy to reuse libraries, so Rustaceans are able to write smaller projects that are assembled from number of packages.

### Generating a Random Number
Now that we have added the `rand` crate to _Cargo.toml_, lets start using `rand`. The next step is to update _src/main.rs_.

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

First, we add `use::rand::Rng`. The `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.

Next, we are adding two lines in the middle. The `rand::thread_rng` function will give us the particular random number generator that we are going to use. One that is local to the current thread of execution and seeded by the operating system. Then we call the `gen_range` methods on the random number generator. This method is defined by the `Rng` trait that we brought into scope with the `use rand::Rng` statement. The `gem_range` method takes two numbers as arguments and generates a random number between them. It is inclusive on the lower bound but exclusive on the upper bound, so we need to specify 1 and 101 to request a number between 1 and 100.

> **Note:** You won't just know which traits to use and which methods to call from a crate. Instructions for using a crate are in his documentation. Another neat feature of Cargo is that you can run `cargo doc --open` command, which will build documentation provided by all of your dependencies locally and open it in your browser. If you are interested in other functionality in the `rand` crate, run `cargo doc --open` and then click `rand` in the sidebar.

The second line that we added to the middle of the code prints the secret number. This is useful while we are developing the program to be able to test it, but we will delete it from the final version. It is not much of a game if the program prints the answer as soon as its starts.

Try to run the program a few times, and the random guess number will be different each time. The number should be between 1 and 100. Great Job!.

### Links
+ [Rand crate](https://crates.io/crates/rand)
+ [Semantic Versioning](http://semver.org/)
+ [Cargo.io](https://crates.io/)

## 4. Comparing the Guess to the Secret Number
Now that we have user input and a random, we can compare them. That step is shown below, but keep in mind that this code won't compile quite yet, as we will explain.

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Notice that we add `use std::cmp::Ordering` into the scope of the program from the standard library.Like `Result`, `Ordering` is another enum, whose variants are `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.

The we add some line that use the `Ordering` type. The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with. In this case we will compare the `guess` against `secret number`. Then it returns a variant of the `Ordering` enum we brought into scope with the `use` statement. We use a `match` expression to decide what to do next based which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

A `match` expression is made up of **arms**. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm's pattern. Rust takes the value given to `match` and look through each arm's pattern in turn. The `match` construct and patterns are powerful features in Rust that let you express a variety of situation your code might encounter and make sure that you can handle them all. These features will be covered in Chapter 6 and Chapter 18 respectively.

Let's walk through an example of what would happen with the `match` expression used here: Say that the user has guessed 50 and the randomly generated secret number is 38. When the code compares 50 to 38, the `cmp` method will return `Ordering::Greater`, because 50 is greater than 38. The `match` expression gets the `Ordering::Greater` value and starts checking each arm's pattern. It looks at the first arm's pattern, `Ordering::Less`, and sees that the values moves to the next arm. The next arm's pattern, `Ordering::Greater`, does match `Ordering::Greater`, Boom! The associated code in that arm will execute and print `Too big!` to the screen. The `match` expression ends because it has no need to look at the last arm in this scenario.

However, the last code won't compile yet, and will send the next error:

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integer
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `guessing_game`.
```

The code of the error states that there are _mismatche types_. Rust has a strong, static type system. However, it also has type inference. When we wrote `let mut guess = String::new()`, Rust was able to infer that `guess` should be a `String` and did not make us write the type. The `secret_number`, on the other hand, is a number type. A few number types can have a value between 1 and 100:

+ `i32` is a 32-bit number
+ `u32` is an unsigned 32-bit number,
+ `i64` us a 64-bit number

Rust defaults to an `i32`, which is the type of `secret_number` unless you add a type information elsewhere that would cause Rust to infer a different numerical type. The reason for the error is that Rust cannot compare a string and a number type.

Ultimately, we want to convert the `String` the program reads as input into a real number type so we can compare it numerically to the secret number. We can do that by adding the following two lines in the `main` function body:

```rust
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```
 We create a variable named `guess`. But wait, does not the program already have a variable named `guess`? It does, but Rust allows us to **shadow** the previous value of `guess` with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess_num` for example.

 We bind `guess` to the expression `guess.trim().parse`. The `guess` in the expression refers to the original `guess` that was a `String` with the input in it. The `trim` method on a string instance will eliminate any white space at the beginning and end. Although `u32` can contain only numerical characters, the user must press enter to satisfy `read_line`. When the user presses enter, a newline character is added to the string. For example, if the user types 5 and presses enter, `guess`  looks like this `5\n`. The `\n` represents "newline", the result of pressing enter. The `trim` method eliminates `\n`, resulting just `5`.

 The `parse` method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using `let guess: u32`. the colon `:` after `guess` tells Rust we will annotate the variable's type. Remember that Rust has a few built-in numbers types, as we see before, so please keep in mind that the comparison will be between two values of the same numerical type.

 The call to `parse` could easily cause an error. If, for example, the string contained `A%`, there would be no way to convert that to a number. Because it might fail, the parse method returns a `Result` type, much as the `read_line` methods does. We will treat this `Result` the same wayby using the `expect` method again. If `parse` returns an `Err` from the `Result` variant because it could not create a number from the string, the `expect` call will crash the game and print the message we give it. If `parse` can successfully convert the string to a number, it will return the `Ok` variant of `Result`, and `expect` will return the number that we want from the `Ok` value.

Let's run the program now!

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!

```

Nice! Even though spaces were added from before the guess, the program still figured out that the user guess 76. Run the program a few times to verify the different behavior with different kinds of input. Guess the number correctly, guess a number that is to high, and guess a number that is to low.

We have most of the game working now, but the user can make only one guess. Let's change that by adding a loop.

### Links
+ [match control flow operator](https://doc.rust-lang.org/book/ch06-02-match.html)
+ [parse method](https://doc.rust-lang.org/std/primitive.str.html#method.parse)

## 5. Allowing Multiple Guesses with Looping
The `loop` keyboard creates an infinite loop. We will add that now to give users more chances at guessing the number:

```rust
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```
As you can see, we have moved everything into a loop from the guess input prompt onward. Be sure to indent the lines inside the loop another four spaces each and run the program again. Notice that there is a new problem because the program is doing exactly what we told it to do. Ask afor another guess forever!.

The user could always interrupt the program by using the keyboard shortcut `Ctrl - C`. But there is another way to escape this insatiable monster, as mentioned in the `parse` discussion in the last section. If the user enters a non-number answer, the program will crash. The user can take advantage of that order to quit, as shown here.

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
```

Typing `quit` actually quits the game, but so will any other non-number input. However, this is sub optimal to say the least. We want the game to automatically stop when the correct number is guessed.

### Quitting After a Correct Guess
Let's program the game to quit when the user wins by adding a `break` statement.

```rust
// --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Adding the `break` line after `You win!` makes the program exit the loop when the user guesses the secret number correctly. Exiting the loop also means exiting the program, because the loop is the last part of the `main`.

### Handling Invalid Input
To further refine the game's behavior, rather than crashing the program when the user inputs a non-number, lets make the game ignore a non-number so the user can continue guessing. We can do that by altering the line were `guess` is converted from `String` to `u32`, as shown the next snippet.

```rust
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```

Switching from an `expect` call to a `match` expression is how you generally move from crashing on an error to handling the error. Remember that the `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` and `Err`. We are using a `match` expression here, as we did with the `Ordering` result of the `cmp` method.

If `parse` is not able to turn the string into a number it will return an `Err` value that contains more information about the error. The `Err` values does not match the `Ok(num)` pattern in the first `match` arm, but it does match the `Err(_)` pattern in the second arm. The underscore `_` is a catchall value. In this example we are saying we want to match all `Err` values, no matter what information they have inside them. So the program will execute the second arm's code, `continue`, which tell the program to go to the next iteration of the `loop` and ask for another guess. So, effectively, the program ignores all errors that `parse` might encounter.

Now everything in the program should work as expected. Awesome, just a tiny final tweak, we will finish the guessing game. Recall that the program is still printing the secret number. That worked well for testing, but it ruins the game. Let's delete the `println!` that outputs the secret number. So the final version is:

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Summary
At this point, you have successfully built the guessing game. This project was a hands-on way to introduce you many Rust concepts

+ `let` variables
+ `match` methods
+ Associated functions
+ Use of external crates
+ Use of macros
+ Reference syntax
+ Use of the standard library
+ Loops
+ Data types
