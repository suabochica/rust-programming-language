# Chapter 3: Common Programming Concepts
This chapter covers concepts that appears in almost every programming language and how they work in Rust. Many programming languages have much in common at their core. None of the concepts presented in this chapter are unique to Rust, but we will discuss them in the context of Rust and explain the conventions around these concepts.

Specially, you will learn about variables, basic types, functions, comments and control flow. These foundations will be in every Rust program, and learning them early will give you a strong core to start from.

> **Keywords**
> The Rust language has a set _keywords_ that are reserved for use by the language only, much as in the other languages. Keep in mind that you cannot use these words as names of variables or functions. Most of the keywords have special meaning, an you will be using them to do various task in your Rust programs.

## Index
1. Variables and Mutability
2. Data Types
3. Functions
4. Comments
5. Control Flow

## 1. Variables and Mutability
By default variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers. However, you still have the option to make your variables mutable. Let's explore how and why Rust encourages you to favor immutability and why sometimes you might want to opt out.

When a variable is immutable, once a value is bound to a name, you cannot change that value. To illustrate this, let's generate a new project called _variables_ in your project directory by using `cargo new variables`.

Then, in the new variables folder replace the content of `src/main.rs` with:

```rust
 fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Save an run the program using `cargo run`. You should receive an error message, as shown in this output.

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

This example show the compiler helps you find errors in your program. Even though compiler errors can be frustrating, they only mean your program is no safely doing what you want it to do yet. They do not mean that you are not a good programmer, compiler errors are part of the learning process.

The error message indicates that the cause is: `cannot assign twice to immutable variable x`, because you tried to assign a second value to the immutable `x` variable.

It is important that we get compile-time errors when we attempt to change a values that we previously designated as immutable because this very situation can lead to bugs. If one part of our code operates on the assumption that a values will never change and another part of our code changes that values, it is possible that the first part of the code won't do what it was designed to do. The cause of this kind of bug can be difficult to track down after the fact, especially when the second piece of code changes the values _sometimes_.

In Rust, the compiler guarantees that when you state that a values won't change, it really won't change. That means that when you are reading and writing code, you don't have to keep track of how and where a value might change. Your code is thus easier to reason through.

But mutability can be very useful. Variables are immutable only by default. As we can saw in the chapter 2, we can make variable mutable with the `mut` keyword in front of the variable name. In addition to allowing this value to change, `mut` conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable's value.

For example, lets change `src/main.rs` to:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

When we run the program now, we get:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

We are allowed to change the value that `x` binds from 5 to 6 when `mut` is used. In some cases, you will want to make a variable mutable because it makes the code more convenient to write that if it had only immutable variables.

There are multiple trade-offs to consider in addition to prevention of bugs. For example, in cases where you are using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming styles may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

### Differences Between Variables and Constants.
Being unable to change the value of a variable might have reminded you of another programming concept that most other languages have. **constants**. Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you are not allowed use `mut` with constants. Constants are not just immutable by default — they are always immutable —.

You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated. We are about to cover types and type annotations in **Data Types**, so don't worry about details right now. Just know that you must always annotate the type.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constant may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

Here is an example of a constant declaration where constant's name is `MAX-POINTS` and its value is set to 100,000. The Rust naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability.

```
const MAX_POINTS: u32 = 100_000;
```

Constants are valid for the entire time a program runs, within the scope they were declared in, making them a useful choice for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn or the speed of light.

Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.

### Shadowing
We introduce the concept of **shadowing** in the implementation of the guessing game at the moment of compare the guess number against the secret number. Basically we declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable. Rustaceans say that the first variable is _shadowed_ by the second, which means that the second variable's value is what appears when the variable is used. We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword as follows.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
}

```

This program first binds `x` to the value of 5. Then it shadows `x` by repeating `let x =`, taking the original value and adding `to the value of `x` is then 6. The third `let` statement also shadows `x`, multiplying the previous value by 2 to give `x` a final value of 12. When we run this program the output is the following.

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12
```

Shadowing is different from marking a variable as `mut`, because we will get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that because we are effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, but we really want to store that input as a number:

```rust
#![allow(unused_variables)]
fn main() {
  let spaces = "   ";
  let spaces = spaces.len();
}
```

This construct is allowed because the first `spaces` variable is a string type and the second `spaces` variable, which is a brand-new variable that happens to have the same name as the first one, is a number type. Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`. Instead, we can reuse the simpler `spaces` name. However, if we try to use `mut` for this, as shown here, we will get a compile-time error.

```rust
#![allow(unused_variables)]
fn main() {
  let mut spaces = "   ";
  let spaces = spaces.len();
}
```

The error says we are not allowed to mutate a variable's type:

```
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

Now that we have explored how variables work, let's look at more data types the can have.

## 2. Data Types
Every value in Rust is of a certain **data type**, which tells Rust what kind of data is being specified so it knows how to work with that data. We will look at two data type subsets, _scalar_ and _compound_.

Keep in mind that Rust is a **statically typed language**, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value an how we use it. In cases when many types are possible, such as when we converted a `String` to a numeric type using `parse` in the comparison of the guess number against the secret number, we must add a type annotation like this:

```rust
let guess: u32 = "42".parse().expect("Not a number");
```

If we don't add the type annotation here, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use.

```
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type

```

You will see different type annotations for other data types.

### Scalar Types
A _scalar_ type represents a single value. Rust has four primary scalar types.

+ Integers
+ Floating-point numbers
+ Characters
+ Booleans

You may recognize these from other programming languages, let's jump into how they work in Rust.

#### Integer Types
An _integer_ is a number without a fractional component. We used one integer type in Chapter 2, the `u32` type. This type declaration indicates that the value it is associated with a should be an unassigned integer that takes up 32 bits of space. The next table shows the built-in integer types in Rust.

| **Length** | **Signed** | **Unsigned**  |
|------------|------------|---------------|
| 8-bit      | `i8`       | `u8`          |
| 16-bit     | `i16`      | `u16`         |
| 32-bit     | `i32`      | `u32`         |
| 64-bit     | `i64`      | `u64`         |
| 128-bit    | `i128`     | `u128`        |
| arch       | `isize`    | `usize`       |

Each variant can be either signed or unsigned and has an explicit size. _Signed_ and _unsigned_ refer to whether it is possible for the number to be negative or positive — in other words, whether the number needs to have a sign with it signed or whether it will only ever be positive and can therefore be represented without a sign, unsigned. It is like writing numbers on paper. When the sign matters, a number is shown with a plus or a minus sign. However, when it is safe to assume the number is positive, it is shown with no sign. Signed numbers are stored using two's complement representation.

Each signed variant can store numbers from -2ˆ(n-1) to 2ˆ(n-1) - 1 inclusive, where _n_ is the number of bits that variant uses. So an `i8` can store number from -2ˆ7 to 2ˆ7 -1 (i.e. -128 to 127). Unsigned variants can store number from 0 to 2ˆ(n-1), so a `u8` can store numbers from 0 to 2^8 - 1, which equals to 0 from 0 to 255.

Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on. 64 bits if you are on a 64-bit architecture and 32 bits if you are on a 32-bit architecture.

You can write integer literals in any of the forms shown in the next table. Note that all number literals except the byte literal allow a type suffix, such as `57u8`, and `_` as a visual separator, such as `1_000`.

| **Number**      | **Example**   |
|-----------------|---------------|
| Decimal         | 98_222        |
| Hex             | `0xff`        |
| Octal           | `0o77`        |
| Binary          | `0b1111_0000` |
| Byte (`u8`only) | `b'A`         |

So how do you know which type of integer to use? If you are unsure, Rust's defaults a re generally good choices, and integer types default to `i32`. This type is generally the fastest, even on 64-bit systems. The primary situation in which you would use `isize` or `usize` is when indexing some sort of collection.

> **Integer Overflow**
> Let's say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value a outside of that range, such as 256, _integer overflow_ will occur. Rust has some interesting rules involving this behavior. When you are compiling in debug mode, Rust includes checks for integer overflow that cause your program to _panic_ at runtime if this behavior occurs. Rust uses them panicking when a program exits with an error. We will dicuss panics in Chapter 9.

> When you are compiling in release mode with the `--release` flag, Rust does not include check for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two's complement wrapping. In short, values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type can hold. In the case of a `u8`, 256 becomes 0, 257 becomes 1, and so on. The program won't panic but the variable will have a values that probably is not what you were expecting it to have. Relying on integer overflow's wrapping behavior is considered an error. If you want to wrap explicitly, you can use the standard library type `Wrapping`.

#### Floating-Point Types
Rust also has two primitive types for _floating-point numbers_, which are number with decimal points. Rust's floating-points types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs it is roughly the same speed as `f32` but is capable of more precision.

Here is an example that shows floating-point numbers in action:

```
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single precision float, and `f64` has double precision.

#### Numeric Operations
Rust supports the basic mathematical operations you would expect for all of the number types. Addition, subtraction, multiplication, division and remainder. The following code shows how you would use each one in the `let` statement.

```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 + 4.3;

    // multiplication
    let product = 5 * 30;

    // division
    let quotient = 75.5 / 12.3;

    // remainder
    let remainde = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable.

#### The Boolean Type
As in most other programming languages, a Boolean type in Rust has to possible values: `true` and `false`. Booleans are one byte in size. The Boolean type in Rust is specified using `bool`. For example:

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as an `if` expression. We will cover how `if` expressions work in Rust in the **Control Flow** section.

#### The Character Type
So far we have worked only with numbers, but Rust support letters too. Rust's `char` type is the language's most primitive alphabetic type, and the following code shows one way to use it.

> **Note:** The `char` literals are specified with single quotes `''`, as opposed to string literals, which use double quotes `""`.

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more that just ASCII. Accented letters, Chinese, Japanese and Korean characters, emojis, and, zero-width space are all valid `char` values in Rust. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a character is not really a concept in Unicode, so your human intuition for what a character is may not match up with what `char` is in Rust. We will discuss the topic in detail in "Storing UTF-8 Encoded Text with Strings" in Chapter 8.

### Compound Types
_Compound types_ can group multiple values into one type. Rust has two primitive compound types:

+ Tuples
+ Arrays

#### The Tuple Type
A tuple is a general way of grouping together a number of values with variety of types into one compound type. Tuples have a fixed length, once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same. We have added optional type annotations in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds the entire tuple, because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value like this.

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y); //Prints 6.4
}
```

This program first creates a tuple and binds it to the variable `tup`. It then uses a pattern with `let` to take `tup` and turn it into three separate variables, `x`, `y`. and `z`. This is called **destructuring**, because it breaks the single tuple into three parts.

In addition to destructuring through pattern matching, we can access a tuple element directly bu using a period `.` followed by the index of the value we want to access. For example.

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```
This program creates a tuple, `x`, and then makes new variables for each element by using their respective indices. As with most programming languages, the firs index in a tuple is 0.

#### The Array Type
Another way to have a collection of multiple values is with an _array_. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different for arrays in some other languages because arrays in Rust have a fixed length, like tuples.

In Rust, the values going into an array are written as comma separated list inside square brackets.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than the heap – We will discuss the stack and the heap more in Chapter 4 – or when you want to ensure you always have a fixed number of elements. An array is not as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow and shrink in size. If you are unsure whether to use an array or vector, you should probably use a vector.

An example of when you might want to use an array rather than a vector is in a program that needs to know the names of the month of the year. It is very unlikely that such a program will need to add or remove months, so you can use an array because you know it will always contain twelve elements.

```rust

#![allow(unused_variables)]
fn main() {
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```

You would write an array's type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array like so.

```rust
#![allow(unused_variables)]
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```
Here, `i32` is the type of each element. After the semicolon, the number 5 indicate the array contains five elements.

Writing an array's type this way looks similar to an alternative syntax for initializing an array. If you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here.

```rust

#![allow(unused_variables)]
fn main() {
let a = [3; 5]; // Prints [3, 3, 3, 3, 3]
}

```

##### Accessing Array Elements
An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `, because that is the value at index [0] in the array. The variable `second` will get the value 2 from index [1] in the array.

##### Invalid Array Elements Access
What happens if you try to access an element of an element of an array that is past the end of the array? Say you change the example to the following code, which will compile but exit with an error when it runs.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];

    println!("The value of element is: {}", element);
}

```

Running this code using `cargo run` produces the following result.

```
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/arrays`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:5:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The compilation did not produce any errors, but the program result in a _runtime_ error and did not exit successfully. When you attempt to access an element using indexing, Rust will check that the index you have specified is less than the array length. If the index is greater that or equal to the array length, Rust will panic.

This is the first example of Rust's safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.

## 3. Functions
Functions are pervasive in Rust code. You have already seen one of the most important functions in the language, `main()`, which is the entry point of many programs. You have also seen the `fn` keyword, which allows you to declare new functions.

Rust code use _snake case_ as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words. Here is a program that contains an example function definition.

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Function definitions in Rust start with `fn` and have a set of parentheses after the function name. The curly brackets tell the compiler where the function body begins and ends.

We can call any function we have defined by entering its name followed by a set of parentheses. Because `another_function` is defined in the program, it can be called from inside the `main` function. Note that we defined `another_function` after the `main` function in the source code. We could have defined it before as well. Rust does not care where you define your functions, only they are defined somewhere.

Let's start a new binary project named _functions_ to explore functions further. Place the `another_function` example in the `src/main.rs` and run it. You should see the following output.

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order in which they appear in the `main` function. First "Hello, world!" message prints, and then `another_function` is called and its message is printed.

### Function Parameters
Functions can also be defined to have _parameters_, which are special variables that are part of the function's signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called _arguments_, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function's definition or the concrete values passed in when you call a function.

The following rewritten version of `another_function` show what parameters look like in Rust.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Try running this program, you should get the following output.

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
     Running `target/debug/functions`
The value of x is: 5
```

The declaration of `another_function` has one parameter named `x`. The type of `x` is specified as `i32`. When 5 is passed to `another_function`, the `println!` macro puts 5 where the pair of curly brackets were in the format string.

In function signature, you must declare the type of each parameter. This is a deliberate decision in Rust's design, requiring type annotations in functions definitions means the compiler almost needs you to use them elsewhere in the code to figure out what you mean.

When you want a function to have multiple parameters, separate the parameter declarations with commas, like this.

```
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

This example create a function with two parameters, both of which are `i32` types. The function then prints the values in both of its parameters. Note that function parameters do not all need to be the same type, they just happen to be in this example.

Lets try running this code. Replace the program in the `src/main.rs` and execute `cargo run`, you will get the next output.

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Because we called function with 5 as the value of `x` and 6 is passed as the value of `y**, the two strings are printed with these values.

### Function Bodies Contain Statements and Expressions
Function bodies are made up of a series of statements optionally ending in a expression. So far, we have only covered functions without an ending expression, but you have seen an expression as part of a statement. Because **Rust is an expression-based language**, this is an important distinction to understand. Other languages do not have the same distinctions, so let's look at what statements and expressions are and how their differences affects the bodies of functions.

We have actually already used statements and expressions. _Statements_ are instruction that perform some action and do not return a value. _Expressions_ evaluate to a resulting value. Lets look at some examples.

Creating a variable assigning a value to it with `let` keyword is a statement. As is shown below.

```rust
fn main() {
    let y = 6;
}
```

Function definitions are also statements. The entire preceding example is a statement itself.

Statements do not return values. Therefore, you cannot assign a `let` statement to another variable, as the following code tries to do.

```rust
fn main() {
    let x = (let y = 6);
}
```

When you run this program, you get the next error:

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

`let = 6` statement does not return a value, so there is not anything for `x` to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the values of the assignment. In those languages, you can write `x = y = 6` and have both `x` and `y` have the value 6. That is not the case in Rust.

Expressions evaluate to something and make up most of the rest of the code that you will write in Rust. Consider a simple math operation, such as `5 + 6`, which is an expression that evaluates to the value 11. Expressions can be part of statements. In the last code, the 6 in the statement `let = 6;` is an expression that evaluates to the value 6. Calling a gunction is an expression. Calling a macro is an expression. The block that we use to create new scopes, `{}`, is an expression, for example in the next snippet.

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

This expression

```rust
let y = {
    let x = 3;
    x + 1
};
```

Is a block that, in this case evaluates to 4. That value get bounds to `y` as part of the `let` statement. Note the `x + 1` line without semicolon at the end, which is unlike most of the lines you have seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of a expression, you turn it into a statement, which will then not return a value. Keep this in mind as you explore function return values and expressions next.

### Function with Return Values

Function can return values the code that call them. We do not return values, but we do declare their type after an arrow (`->`). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but the most functions return the last expression implicitly. Here is an example of a function that returns a value.

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, or even `let` statements in the `five()` function — just the number 5 by itself —. That is perfectly valid function in Rust. Note that the function's return type is specified two, a `-> i32`. Try running this code and the output should be.

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
The value of x is: 5
```

This 5 in `five()` is the function's return value which is why the return type is i32. Let's examine this in more detail. There are two important bits. First, the line `let x = five();` shows that we are using the return value of a function to initialize a variable. Because the function `five` returns a 5, that line is the same as the following.

```rust
let x = 5;
```

Second, the `five` function has no parameters and defines the type of the return value, but the body of the function is a lonely 5 with no semicolon because it is an expression whose value we want to return.

Let's look at another example.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x); // Prints 6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Please keep in mind that the body of `plus_one` is an expression if we add a semicolon to this expression to convert it in a statement like this.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x); // Prints 6
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

In compile time the code produce an error as follows:

```
error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
  | |          - help: consider removing this semicolon
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
```

The main error message, "mismatched types", reveals the core issue of this code. The definition of the function `plus_one` says that it will return a `i32`, but statements do not evaluate to a value, which is expressed by `()`, an empty tuple. Therefore, nothing is returned, which contradicts the function definition and results in an error. In this output, Rust provides a message to possible help rectify this issue. It suggest removing the semicolon, which would fix the error.

## 4. Comments
All programmers strive to make their code easy to understand. but sometimes extra explanation is warranted. In these cases, programmers leave notes (a.k.a _comments_) in their source code that compiler will ignore but people reading the source code may find useful. Below an example of comments in Rust.

```rust
// Hello, world!
```

In Rust, comments must start with two slashes and continue until the end of the line. For comments that extend beyond a single line, you will need to include `//` on each line, like this.

```rust
#![allow(unused_variables)]
fn main() {
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
}
```

Comments can also be placed at the end of lines containing code.

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```

But you will more often see them used in this format, with the comment on a separate line above the code it is annotating.

```rust
fn main() {

    // I’m feeling lucky today
    let lucky_number = 7;
}
```

Rust also has another kind of comment, documentation comments, which we will discuss in the "Publishing a Crate to Crates.io" section of Chapter 14.

## 5. Control Flow
Deciding whether or not run some code depending on if a condition is true and deciding to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are:

+ If Expressions
+ Loops

### `if` Expressions
An `if` expression allows you to branch your code depending on conditions. You provide a condition and then state, "If the condition is met, run this block of code. If the condition is not met, do not run this block of code".

Create a new project called `branches` to explore the `if` expression in the `src/main.rs` file, input the following.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```
All `if` expressions start with the keyword `if`, which if followed by a condition. In this case, the condition checks whether or not the variable number has a values less than 5. The block of code we want to execute if the condition is true is placed immediately after the condition inside the curly brackets. Blocks of code associated with the conditions in `if` expressions are sometimes called **arms**, just like the arms in `match` expression that we discussed in the comparison of the guess number against the secret number.

Optionally, we can also include an `else` expression, which we chose to do here, to give the program an alternative block of code to execute should the condition evaluate to false. If you do not provide an `else` expression and the condition is false, the program will just skip the `if` block and move on to the next bit of code.

Try running this code, you should see the following output.

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

Let's try changing the value of the `number` to a value that makes the condition false. If we change that line with `let number = 7` we will look this output.

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

It is also worth nothing that the condition in this code must be a Boolean. If the condition is not a Boolean we will get an error. For example, try running the following code.

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

The `if` condition evaluates to a value of 3 this time, Rust throws an error.

```
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected bool, found integer
  |
  = note: expected type `bool`
             found type `{integer}`
```

The error indicates that Rust expected a Boolean but got an Integer. Unlike languages as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide `if` with a Boolean with as its condition. If we want the `if` code block to run only when a number is not equal to 0, for example, we can change the `if` expression to the following.

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

#### Handle Multiple Conditions with `else if`
You can have multiple conditions by combining `if` and `else`  in an `else if` expression. For example.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This program has four possible paths it can take. After running it, you should see the following output.

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
number is divisible by 3
```

When this program executes, it checks each `if` expression in turn and executes the first body for which the condition holds true. Note that even though 6 is divisible by 2, we do not see the output `number is divisible by 2`, nor do we see the `number is not divisible by 4, 3, or 2` text from the `else` block. That is because Rust only executes the block for the first true condition, and once it finds one, it does not even check the rest.

Using too many `else if` expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called `match` for these cases.

#### Using `if` in `let` Statement
Because `if` is an expression, we can use it on the right side of a `let` statement, as we see in the next snippet.

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

The `number` variable will be bound to a value based on the outcome of the `if` expression. Run this code to see what happens.

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/branches`
The value of number is: 5
```

Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expression. In this case, the value of the whole `if` expression depends on which block of code executes. this means the values that have the potential to be results from each arm of the `i` must be the same type. In the last code, the results of both the `if` arm and the `else` arm were `i32` integers. If the types are mismatched, as in the following code we will get an error.

```rust
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```
When we try to compile this code, we will get an error. The `if` and `else` arms have value types that are incompatible, and Rust indicates exactly where to find the problem in the program.

```
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integer, found &str
  |
  = note: expected type `{integer}`
             found type `&str`
```

The expression in the `if` block evaluates to an integer, and the expression in the `else` block evaluates to a string. This won't work because variables must have a single type. Rust needs to know at compile time what type the `number` variable is, definitively, so it can verify at compile time that its type is valid everywhere we use `number`. Rust would not be able to do that if the type of `number` was only determined at runtime. The compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.


### Repetition with Loops
It is often to execute a block of code more than once. For this task, Rust provides several _loops_. A loop runs through the code inside the loop body to the end and then starts immediately back at the beginning. To experiment with loops, let's make a new project called `/loops`. Rust has three kinds of loops:

+ Loop
+ While
+ For

#### Loop
The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it stop.

As an example, change `src/main.rs` file in your `/loops` directory to look like this.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

The symbol `^C` represents where you pressed `Ctrl + c`. You may o may not see the word `again!` printed after the `^C`, depending on where the code was in the loop when it received the interrupt signal.

Fortunately, Rust provides another reliable way to break out of a loop. You can place the `break` keyword within the loop to tell the program when to stop executing the loop. Recall that we did this in the guessing game in the quitting after a correct guess step, to exit the program when the user won the game by guessing the correct number.


##### Returning Values from Loops
One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code. To do this, you can add the value you want returned after the `break` expression you use to stop the loop. That value will be return out of the loop so you can use it, as shown here.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

Before the loop, we declare a variable name `counter` and initialize it to 0. Then we declare a variable named `result` to hold the value returned from the loop. On every iteration of the loop, we add 1 to the `counter` variable, and then check whether the counter is equal to 10. When it is, we use the `break` keyword with the value `counter * 2`. After the loop, we use a semicolon to end the statement that assigns the value to `result`. Finally, we print the value in `result`, which in this case is 20.

#### While
It is often useful for a program to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls `break`, stopping the loop. This loop type could be implemented using a combination of `loop`, `if`, `else`, and `break`. You could try that now in a program if you would like.

However, this pattern is so common that rust has a built-in language construct for it, called a `while` loop. The next program loops three times, counting down each time, and then,after the loop, it prints another message and exits.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else` and `break`, and it is clearer. While a condition holds true, the code runs, otherwise, it exits the loop.

#### For
You could use the `while` construct to loop over the elements of a collection, such as an array. For example.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

Here the code counts up through the elements in the array. It starts at index 0, and then loops until it reaches the final index in the array. Running this code will print every element in the array.

```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

All five array values appear in the terminal, as expected. Even though the `index` will reach a value of 5 at some point, the loop stops executing before trying to fetch a sixth value form the array.

But this approach is error prone. We could cause the program to panic if the index length is incorrect. It is also slow, because the compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.

As more concise alternative, you can use a `for` loop and execute some code for each item in a collection. A `for` loop looks like the below snippet.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

When we run this code, we will see the same output as in the `while` example. More importantly, we have now increased the safety of the code and eliminated the change of bugs that might result form going beyond the end of the array or not going far enough and missing some items.

For example, in the example of `while`, if you removed an item from the array but forgot to update the condition, to `while index < 4`, the code would panic. Using the `for` loop, you would not need to remember to change any other code if you changes the number of values in the array.

The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a `while` loop sample, most Rustaceans would use a `for` loop. The way to do that would be to use a `Range`, which is a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number.

Here is what the countdown would look like using a `for` loop and another method we have not yet talked about, `rev`, to reverse the range.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

This code is nicer.

### Links
+ [If expression](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

## Summary
You made it! that was a sizable chapter, you learned about variables, scalar, compound data types, functions, comments, if expressions, and loops. If you want to practice with the concepts discussed in this chapter, try building a program to do the following.

+ Convert temperatures between Fahrenheit and Celsius.
+ Generate nth Fibonacci number.
+ Print the lyrics to the Christmas carol "The twelve days of Christmas" taking advantage of the repetition in the song.

When you have ready to move one, we will talk about the concept in Rust that does not commonly exist in other programming languages. Ownership.
