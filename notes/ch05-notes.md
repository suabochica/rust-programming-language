# Chapter 5: Using Structs to Structure Related Data
A _struct_ or _structure_ is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you are familiar with object-oriented language, a _struct_ is like an object's data attribute. In this chapter, we will compare and contrast tuples with structs, demonstrate how to use structs, and discuss how to define methods and associated functions to specify behavior associated with a struct's data. Structs and enums are the building blocks for creating new types in your program's domain to take full advantage of Rust's compile time type checking.


## Index
1. Defining and Instantiating Structs
2. An Example Program Using Structs
3. Method Syntax

## 1. Defining and Instantiating Structs
Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you will name each piece of data so it is clear what the values mean. As a result of these names, structs are more flexible than tuples. You do not have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A struct's name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call _fields_. For example, the next snippet shows a struct that stores information about a user account.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
}
```

To use a struct after we have defined it, we create an _instance_ of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing `key: value` pairs, where the keys are the names of the fields and the values are data we want to store in those fields. We do not have to specify the fields in the same order in which we declared them in the struct. In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type. For example, we can declare a particular user as shown the code below.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };
}
```

To get a specific value from a struct, we can use dot notation. If we wanted just this user's email address, we could use `user1.email` whenever we wanted to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Next code shows how to change the value in the `email` field of a mutable `User` instance.

```rust
#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
}
```

Note that the entire instance must be mutable. Rust does not allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

Next example shows a `build_user` function that returns `User` instance with the given email and username. The `active` field gets the value of `true`, and the `sign_in_count` gets a value of `1`.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  fn build_user(email: String, username: String) -> User {
      User {
          email: email,
          username: username,
          active: true,
          sign_in_count: 1,
      }
  }
}
```

It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the `email` and `username` field names and variables is a bit tedious. If the struct had more fields, repeating each name would get even more annoying. Luckily, there is a convenient shorthand.

### Using the Field Init Shorthand when Variables and Fields Have the Same Name
Because the parameter names and the struct field names are exactly the same, we can use the _field init shorthand_ syntax to rewrite `build_user`  so that it behaves exactly the same but does not have the repetition of `email` and `username`, as shown below.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  fn build_user(email: String, username: String) -> User {
      User {
          email,
          username,
          active: true,
          sign_in_count: 1,
      }
  }
}
```
Here, we are creating a new instance of the `User` struct, which has a field named `email`. We want to set the `email` fields's value to the value in the email parameter of the `build_user` function. Because the `email` field an the email parameter have the same name, we only need to write `email` rather than `email:email`.

### Creating Instance From Other Instances With Struct Update Syntax
It is often useful to create a new instance of a struct that uses most of an old instance's values but changes some. You will do this using _struct update syntax_.

First, the next snippet shows how we create a new `User` instance in `user2` without the update syntax. We set new values for `email` and `username` but otherwise use the same values from `user` that we created before.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };

  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),
      active: user1.active,
      sign_in_count: user1.sign_in_count,
  };
}
```
Using struct update syntax, we can achieve the same effect with less code, as shown in the next sample. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };

  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),
      ..user1
  };
}
```

The last code also creates an instance in `user2` that has a different value for `email` and `username` but has the same values for the `active` and `sign_in_count` fields from `user1`.

### Using Tuple Strutcs Without Named Fields to Create Different Types
You can also define struct that look similar to tuples, called _tuple structs_. Tuple struct have the added meaning the struct name provide but do not have names associated with their fields. Rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from the other tuples, and naming each field as in a regular struct world be verbose or redundant.

To define a tuple struct, start with the `struct` keyword and the struct name followed by the types in the tuple. For example, here are definitions and usages of two tuple structs name `Color` and `Point`.

```rust

#![allow(unused_variables)]
fn main() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```

Note that the `black` and `origin` values are different types, because they are instances of different tuple struts. Each struct you define is it own type, even though the fields within the struct have the same types. For example, a function that takes a parameter of `Color` cannot take a `Point` as an argument, even though both types are made up of three `i32` values. Otherwise, tuple struct instances behave like tuples. You can destructure them into their individual pieces, you can use a `.` followed by the index to access and individual value, and so on.

### Unit-Like Structs Without Any Fields
You can also define structs that do not have any fields. There are called _unit-like structs_ because the behave similarly to `()`, the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but do not have any data that you want to store in the type itself.

### Ownership of Struct Data
In the `User` struct definition, we use the owned `String` type rather that the `&str`. string slice type. This is a deliberate choice because we want instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It is possible for structs to store references to data owned by something else, but to do so requires the use of _lifetimes_, a Rust feature that we will discuss in chapter 10. Lifetimes ensure that the data referenced by struct is valid for as long as the struct is. Let's say you try to store a reference in a struct without specifying lifetimes, like the next example.

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

The compiler will complain that it need lifetime specifiers

```
error[E0106]: missing lifetime specifier
 -->
  |
2 |     username: &str,
  |               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 -->
  |
3 |     email: &str,
  |            ^ expected lifetime parameter
```

In chapter 10 we will discuss how to fix these error so you can store references in structs, but for now, we will fix errors like these using owned types like `String` instead of references like `&str`.

## 2. An Example Program Using Structs
To understand when we might want to use structs, let's write a program that calculate the area of a rectangle. We will start with single variables, and then refactor the program until we are using structs instead.

Let's make a new binary project with Cargo called `rectangles` that will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle. The next snippet is one way of achieve our goal.

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
```

When you run `cargo run` we get:

```
The area of the rectangle is 1500 square pixels.
```

Even though the last code works and figures out the area of the rectangle by calling the `area` function with each dimensions, we can do better. The width and the height are related to each other because together they describe one rectangle.

The issue with this code is evident in the signature of `area`:

```rust
fn area(width: u32, height: u32) -> u32 {}
```

The `area` function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters. The parameters are related, but that is not expressed anywhere in out program. It would be more readable and more manageable to group width and height together. We have already discussed one way we might to do that in the tuple type.

### Refactoring with Tuples
The version of out program with tuples is like this.

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

In one way, this program is better. Tuples let us add a bit of structure, and we are now passing just one argument. But in another way, this version is less clear. Tuples do not name their elements, so our calculation has become more confusing because we have to index into the parts of the tuple.

It does not matter if we mix up width and height for the area calculation, but if we want to draw the rectangle on the screen, it would matter. We would have to keep in mind that `width` is the tuple index `0` and `height` is the tuple index `1`. If someone else worked on this code, they would have to figure this out and keep it in mind as well. It would be easy to forget or mix up these values and cause errors, because we have not conveyed the meaning of our data in our code.

### Refactoring with Tuples: Adding More Meaning
We use structs to add meaning by labeling the data. We can transform the tuple we are using into a data type with a name for the whole as well as name for the part, as shown the next code.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Here we have defined a struct and named it `Rectangle`. Inside the curly brackets, we defined the fields `width` and `height`, both of which type `u32`. The in `main`, we created a particular instance of `Rectangle` that has a width of 30 and a height of 50.

Our `area` function is now defined with one parameter, which we have named `rectangle`, whose type is an immutable borrow of a struct `Rectangle` instance. As mentioned in chapter 4, we want to borrow the struct rather than take ownership of it. This way, `main` retains its ownership and can continue using `rect1`, which is the reason we use the `&` in the function signature and where we call the function.

The `area` function accesses the `widht` and `height` fields of the `Rectangle` instance. Our function signature for `area` now says exactly what we mean: calculate the area of `Rectangle`, using its `width` and `height` fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rahter than using the tuple index values of `0` and `1`. This is a win for clarity.

### Adding Useful Functionality with Derived Traits
It would be nice to be able to print an instance of `Rectangle` while we are debugging our program and see the values for all its fields. The next snippet tries using `println!` macro as we have used in previous chapters. This won't work, however.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

When we compile this code, we get an error whit this core meesage.

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting knows as `Display`. Output intended for direct end user consumption. The primitive types we have seen so far implemented `Display` by default, because there is only one way should format the output is less clear because there are more display possibilities like use of commas, print the curly brackets, show all the fields among others. Rust, does not try to guess what we want, and structs do not have a provided implementation of `Display`.

If we continue reading errors, we will find this helpful note,

```
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Let's try this suggestion. The `println!` macro call will now look like `println!(rect1 is {:?}, rect1)`. Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. The `Debug` trait enables us to print our struct in a way that is useful for developers so we can see its values while we are debugging our code.

Compile the code with this change. Drat! We still get an error.

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
```

But again, the compiler gives us a helpful note.

```
= help: the trait `std::fmt::Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
```

Rust does include the functionality to print our debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the annotation `# [derive(Debug)]` just before the struct definition, as shown next.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

Now when we run the program we won't get any errors, and we will see the following output.

```
rect1 is Rectangle { width: 30, height: 50 }
```

Nice! It isnot the pretties output, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it is useful to have output that is a bit easier to read. In those case, we can use `{:#?}` instead of `{:?}` in the `println!` string. When we use `{:#?}` style i the example, the output will like this:

```
rect1 is Rectangle {
    width: 30,
    height: 50
}
```

Rust has provided a number of traits for us to use with the `derive` annotation that can add useful behavior to our custom types. Those traits and their behaviors are listed in the appendix C. We will cover how to implement these traits with custom behavior as well as how to create your own traits in chapter 10.

Our `area` function is very specific. It only computed the area of rectangles. It would be helpful to tie this behavior more closely to our `Rectangle` struct. because it won't work with any other type. Let's look at how we can continue refactor this code by turning the `area` function into an area _method_ defined on our `Rectangle` type.

## 3. Method Syntax

### Defining Methods

### Methods With More Parameters

### Associated Functions

### Multiple `impl` Blocks

## Summary
