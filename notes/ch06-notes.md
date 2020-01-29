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

### Patterns that Bind to Values

### Matching with `Option<T>`

### Matches are Exhaustive

### The `_` Placeholder

## 3. Concise Control Flow With If Let

## Summary
