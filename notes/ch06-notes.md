# Chapter 06: Enums and Pattern Matching
In this chapter, we will look _enumerations_. Enums allow you to define a type by enumerating its possible _variants_. First we will define and use enum to show how an enum can encode meaning along with data. Next, we will explore a particularly useful enum, called `Option`, which expresses that a value can be either something or nothing. Then we will look at how **pattern matching** in the `match` expression makes it easy to run different code for different values of an enum. Finally we will cover how the `if let` construct is another convenient and concise idiom available to you handle enums in your code.

Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to _algebraic data types_ in functional languages, such as OCaml and Haskell.

## Index
1. Defining an Enum
2. The Match Control Flow Operator
3. Concise Control Flow With If Let

## 1. Defining an Enum
Let's look at a situation we might want to express in code and see why enums are useful and more appropriate that structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. There are the only possibilities for an IP address that our program come across. We can _enumerate_ all possible variants, which is where enumeration gets its name.

Any IP address can be either a IPv4 or IPv6, but not both at the same time. The property of IP addresses makes the enum data structure appropriate, because enum values can only be one of its variants. Both versions are still fundamentally IP addresses, so the should be treated as the same type when the code is handling situation that apply to any kind of IP address.

We can express this concept in code by defining an `IpAddrKind` enumeration and listing the possible kinds an IP address can be. These are the variants of the enum:

```rust
enum IpAddrKind {
  V4,
  v6,
}
```

`IpAddrKind` is now a custom data type that we can uses elsewhere in our code.

### Enum Values
We can create instances of each of the two variants of `IpAddrKind` like this:

```rust
let four = IpAddrKind::V4,
let six = IpAddrKind::V6,
```

Note the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. The reason this is useful is that now both values are of the same type. We can then, for instance, define a function that takes any `IpAddrKind`:

```rust
fn route(ip_kind: IpAddrKind) {}
```

And we can call this function with either variant:

```rust
route(IpAddrKind::v4);
route(IpAddrKind::v6);
```

Using enums has even more advantages. Thinking more about our IP address type, at the moment we do not have a way to store the actual IP address data. We only know what kind it is. Given that you just learned about structs, you might tackle this problem as shown in the next snippet.

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
}
```

Here we have defined a struct `IpAddr` that has two fields; A `kind` field that is of type `IpAddrKind` and an `address` field of type `String`, We have two instances of this struct. The first home, has the value `IpAddrKind::V4` as its `kind` with associated address data of `127.0.0.1`. The second instance, `loopback` has the other variant of `IpAddrKind::v6` and has an address `::1` associated with. We have used a struct to bundle the `kind` and `address` values together, so now the variant is associated with the value.

We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct, by putting data directly into each enum variant. This new definition of the `IpAddr` enums says that both `V4` and `V6` variants will have associated strings values:

```rust
#![allow(unused_variables)]
fn main() {
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
}
```

We attach data to each variant of the enum directly, so there is no need for an extra struct.

There is another advantage to using enum rather than a struct, each variant can have different types and amounts of associated data. Version four type OP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but still express `V6` addresses as one `String` value, we would not be able to with a struct. Enums handle this case with ease:

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
}

```

We have shown several different ways to define data structures to store version four and version six IP addresses. However, as it turns out, wanting to store IP addresses and encode which kind they are is so common that _the standard library has a definition we can use_. Let's look at how the standard library defines `IpAddr`. It has the exact enum and variants that when have defined and used, but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant.

```rust

#![allow(unused_variables)]
fn main() {
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}
```

This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum. Also, standard library types are often not much more complicated that what you might come up with.

Notes that even though the standard library contains a definition for `UpAddr`, we can still create and use our own definition without conflict because we have not brought the standard library's definition into our scope. We will talk more about bringing types into scope in chapter 7.

Let's look at another example of an enum. Here we have variety of types embedded in its variants.

```rust
fn main() {
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
}
```

The enum has four variants with different types:

- `Quit` has no data associated with it at all.
- `Move` includes an anonymous struct inside it.
- `Write` includes a single `String`.
- `ChangeColor` includes three `i32` values.

Defining an enum with variants such as the ones we list before, is similar to defining different kinds of structs definitions, except the enum does not use the struct keyword and all the variants are grouped together under the `Message` type. The following structs could hold the same data that preceding enum variants hold.

```rust
#![allow(unused_variables)]
fn main() {
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
}
```

But if we used the different structs, which each have their own type, we could not as easily define a function to take any of these kinds of messages as we could with the `Message` enum defined in the first approach, which is a single type.

There is one more similarity between enums and structs: just as we are able to define methods on using `impl`, we are also able to define methods on enums. Here is methods named `call` that we could define on our `Message` enmum.

```rust
#![allow(unused_variables)]
fn main() {
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
}
```

The body of the method would use `self` to get the value that we called the method on. In this example, we have created a variable `m` that has the value `Message::Write(String::from("hello"))`, and that is what `self` will be in the body of the `call` method when `m.call()` runs.

Let's look at another enum in the standard library that is very common and useful: `Option`.

### The `Option` Enum and Its Advantages Over Null Values
In the previous section, we looked at how the `IpAddr` enum let us use Rust's type system to encode more information than just the data into our program. This section explores a case study of `Option`, which is another enum defined by the standard library. The `Option` type is used in many places because encodes the very common scenario in which a value could be something or it could be nothing. Expressing this concept in term of the type system means the compiler can check whether you have handled all the cases you should be handling: this functionality can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in therms of which features you include, but the features you exclude are important too. Rust does not have the null feature that many other languages have. _Null_ is a value that means there is no value there. In languages with null, variables can always be in one of two states, null or not-null.

In his 2009 presentation "Null References: The Billion Dollar Mistake", Tony Hoare, the inventor of null has this to say:

> I call it my billion dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. Bu I could not resist the temptation to put in a null reference
, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

The problem with null values is that if you try to use null as a not-null value, you will get an error of some kind. Because this null or not-null property is pervasive, it is extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

The problem is not really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This is `Option<T>` and it is defined by the standard library as follows:

```rust
#![allow(unused_variables)]
fn main() {
enum Option<T> {
    Some(T),
    None,
}
}
```

The `Option<T>` enum is so useful that it is even included in the prelude. You do not need to bring it into scope explicitly. In addition, so are its variants, you can use `Some` and `None` directly without the `Option::` prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of the type `Option<T>`,

The `<T>` syntax is a feature of Rust we have not talking about yet. It is a generic type parameter, and we will cover generics in chapter 10. For now, all you need to know is that `<T>` means the `Some` variant of the `Option` enum can hold one piece of data of any type. Here are some example of using `Option` values to hold number types and string types:

```rust

#![allow(unused_variables)]
fn main() {
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
}

```

If we use `None` rather than `Some`, we need to tell Rust what type of `Option<T>` we have, because the compiler can't infer the type that the `Some` variant will hold by looking only at a `None` value.

When we have a `Some` value, we know that a value is present and the value is held within the `Some`. When we have a `None` value, in some sense, it means the same thing as null, we do not have a valid value. So why is having `Option<T>` any better that having null?

In short, because `Option<T>` and `T` are different types, the compiler won't let us use an `Option<T>` value as if it were definitely a valid value. For example, this code won't compile because it is trying to add an `i8` to an `Option<i8>`.

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

When we run this code, we get an error message like this:

```
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |

```

In effect, this error message means that Rust does not understand how to ass `i8` and an `Option<i8>`, because they are different types. When we have a value of a type like `i8` in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value. Only when we have an `Option<i8>` do we have to worry about possible not having a value, and the compiler will make sure we handle that case before using the value.

In other words, you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. Generally, this helps catch one of the most common issues with null, assuming that something is not null when it actually is.

Not having to worry about incorrectly assuming a not-null value helps you to be more confident in your code. In order to have a value that can possibly be null, you must explicitly opt in by making handle the case when the value is null. Everywhere, that a value has a type that is not an `Option<T>`, you can safely assume that the value is not null. This was a deliberate design decision for Rust to limit null's pervasiveness and increase the safety of Rust code.

So, how do you get `T` value out of `Some` variant when you have a value of type `Option<T>` so you can use that value? The `Option<T>` enum has a large number of methods that are useful in a variety of situations. Becoming familiar with the methods on `Option<T>` will be extremely useful in your journey with Rust.

In general, in order to use an `Option<T>` value, you want to have code that will handle each variant. You want some code will run only when you have a `Some(T)` value, and this code is allowed to use the inner `T`. You want some other code to run if you have a `None` value, and that code does not have a `T` value. The `match` expression is a control flow construct that does just this when used enums. It will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

### Links
+ [Option<T>](https://doc.rust-lang.org/std/option/enum.Option.html)

## 2. The Match Control Flow Operator
Rust has an extremely powerful control flow operator called `match` that allows you to compare a value against a series of patterns and then execute code based on which patterns matches. Patterns can be made up of literal values, variables names, wildcards and many other thinks. The power of `match` comes from expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Think of a `match` expression as being like a coin-sorting machine. Coins slides down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in an `match`, and at the first pattern the values **fits**, the value fall intro the associated code block to be used during execution.

Because we just mentioned coins, let's use them as an example using `match`. We can write a function that can take an unknown United States coin and, in a similar way as the counting machine, determine which coin it is and return its values in cents.

```rust

#![allow(unused_variables)]
fn main() {
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
}
```

Let's break down the `match` in the `value_in_cents` function. First, we list the `match` keyword followed by an expression, which in this case is the value `coin`. This seem very similar to an expression used with `if`, but there is a big difference. With `if`, the expression needs to return a `boolean` value, but here, it can be a type. The type of `coin` in this example is the `Coin` enum that we defined on the first line.

Next are the `match` arms. An arm has two parts, a _pattern_ and _some code_. The first arm here has a pattern that is value `Coin::Penny` and then the `=>` operator that separates the pattern and the code to run. The code in this case is just the value `1`. Each arm is separated from the next with a comma.

When the `match` expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern does not match the value execution continues to the next arm, much as in a coin-sorting machine. We can have as many arms as we need. In this case we have four arms.

The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire `match` expression.

Curly brackets typically are not used if the match arm code is short. If you want to run multiple line of code in a match arm, you can use curly brackets. For example the next snippet prints "Lucky penny!" every time the method was called with a `Coin::Penny` but would still return the last value of the block, `1`.

```rust

#![allow(unused_variables)]
fn main() {
enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
}
```

### Patterns that Bind to Values
Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out enum variants.

As an example, let's change one of our enum variants to hold data inside it. From 1999 through 2008, the United States minted quarters with different design for eacc of the 50 state on one side. No other coins got state designs, son only quarters have this extra value. We can add this information to our `enum` by changing `Quarter` variant to include a `UsState` value stored inside it, which we have done here:

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Let's imagine that a friend is trying to collect all 50 state quarters. While we sort our loose change by coin type, we will also call out the name of the state associated with each quarter so if it is one our friend does not have the can add it to their collection.

In the match expression for this code, we add a variable called `state` to the pattern that matches values of the variant `Coin::Quarter`. When a `Coin::Quarter` matches, the `state` variable will bind to the values of that quarter's state. Then we can use `state` in the code for that arm, like so:

```rust

#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
}
```

If we were to call `value_in_cents(Coin::Quearter(UsState::Alaska))`, `coin` would match. When we compare the value with each of the match arms, none of the match until we reach `Coin::Quarter(state)`. At that point. the binding for `state` will be the value `UsState::Alaska`. We can then use that binding in the `println!` expression, thus getting the inner state value out of the `Coin` enum variant for `Quarter`.

### Matching with `Option<T>`
We can also handle `Option<T>` using `match` as we did with the `Coin` enum. Instead of comparing coins, we will compare the variants of `Option<T`, but the way that the `match` expression works remains the same.

Let's say we want to write a function that takes a `Option<i32>` and, if there is a value inside, adds 1 to that value. If there is not a value inside, the function should return `None` value and not attempt to perform any operations.

This function is easy to write thanks to `match`, and will look like:

```rust

#![allow(unused_variables)]
fn main() {
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
}
```

Let's examine the first execution of `plus_one`. When we call `plus_one(five)`, the variable `x` in the body of the function will have the value `Some(5)`. We then compare that against each match arm.

```
None => None,
```

The `Some(5)` value does not match the pattern `None`, so we continue to the next arm.

```
Some(i) => Some(i + 1)
```

The `Some(5)` matches with `Some(i)`, we have the same variant, so, the `i` binds to the value contained in `Some` and `i` takes the value of `5`. The code in the match arm is then executed, so we add 1 to the value of `i` and create a new `Some` value with our total 6 inside.

Now let's consider the second call of `plus_one`, where `x` is `None`. We enter the `match` and compare to the first arm.

```
None => None
```

It matches. There is no value to add to, so the program stops and returns the `None` value on the right side of `=>`. Because the first arm matched, no other arms are compared.

Combining `match` and enums is useful in many situations. You will see this pattern a lot in Rust code. `match` against enum, bind a variable to the data inside, and then execute code based on it. It is a bit tricky at first, but once you get used to it, you will wish you had it in all languages, It is consistently a user favorite.
### Matches are Exhaustive
There is one other aspect of `match` we need to discuss. Consider this version of our `plus_one` function that has a bug won't compile.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We did not handle the `None` case, so this code will cause a bug. Luckily, it is a bug Rust knows how to catch. If we try to compile this code, we will get this error.

```
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered

```
Rust knows that we did not cover every possible case and even know which pattern we forgot. Matches in Rust are **exhaustive**, we must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have null thus making the billion-dollar mistake discussed earlier.

### The `_` Placeholder
Rust also has a pattern we can use when we do not want to list all possible values. For example, a `u8` can have valid values of 0 through 255. If we only caer about the values 1, 3, 5, and 7, we do not want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255. Fortunately, we do not have to, we can use the special pattern `_` instead.

```rust

#![allow(unused_variables)]
fn main() {
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
}
```

The `__` pattern will match any value. By putting it after our other arms, the `_` will match all the possible case that are not specifies before it. The `()` is just the unit value, so nothing will happen in the `_` case. As a result, we can say that we want to do nothing for all the possible values that we do not list before the `_` placeholder.

However, the `match` expression can be a bit wordy in a situation in which we care about only _one_ of the cases. For this situation, Rust provides `if let`.

## 3. Concise Control Flow With If Let
The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest. Consider the program below that matches on an `Option<u8>` value but only wants to execute code if the value is 3.

```rust
#![allow(unused_variables)]
fn main() {
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
}
```

We want to do something with the `Some(3)` match but do nothing with any other `Some<u8>` value or the `None` value. To satisfy the `match` expression, we have to add `_ => ()` after processing just one variant, which is a lot of boilerplate code to add.

Instead, we could write this a shorter way using `if let`. The following code behaves the same as the `match` in the last snippet.

```rust

#![allow(unused_variables)]
fn main() {
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
}
```

The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a `match`, where the expression is given to the `match` and the pattern its first arm.

Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that `match` enforces. Choosing between `match` and `if let` depends on what you are doing in your particular situation and whether gaining conciseness is appropriate trade-off for losing exhaustive checking.

In other words, you can think `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values.

We can include `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the `match` expression that is equivalent to the `if let` and `else`. Recall the `Coin` enum definition in last section, where the `Quarter` variant also held a `UsState` value. If we wanted to count all non- quarter coins we see while also announcing the state of the quarters, we could do that with a `match` expression like this.

```rust
let mut count = 0;

match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}", state),
    _ => count += 1,
}
```
Or we can use the `if let` and `else` expression like this:

```rust
let mut count = 0;

if let Coin::Quarter(state) => coin {
   println!("State quarter from {:?}", state);
} else {
    count += 1;
}
```

If you have a situation in which your program has logic that is to verbose to express using `match`, remember that `if let` is in your Rust toolbox as well.

## Summary

We have now covered how to use enums to create custom types that can be one of a set of enumerated values. We have shown how the standard library's `Option<T>` type helps you use the system to prevent errors. When enum values have data inside them, you can use `match` or `if let` to extract and use those values, depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures type safety. The compiler will make certain your functions get only values of the type each function expects.

In order to provide a well organized API to your users that is straightforward to use and only exposes exactly what your users will need, let's now turn to Rust's modules.
