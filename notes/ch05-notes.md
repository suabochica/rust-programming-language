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
_Methods_ are similar to functions. They are declared with the `fn` keyword and their name, they can have parameters and return a values, and they contain some code that is run when they are called from somewhere else. However, methods are different from functions in that they ared defined within the context of a struct, and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

### Defining Methods

Let's change the `area` function that has a `Rectangle` instance as a parameter and instead make an `area` method defined on the `Rectangle` struct, as shown next.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define the function within the context of `Rectangle`, we start an `impl` block. Then we move the `area` function within the `impl` curly brackets and change the first parameter to be `self` in the signature and everywhere within the body. In `main` where we called the `area` function and passed `rect1` as an argument, we can instead use the _method syntax_ to cal the `area` method on out `Rectangle` instance. The method syntax goes after an instance. We add a dot followed by the method name, parentheses, and any argument.

In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle` because Rust knows the type of `self` is `Rectangle` due to this method's being inside the `impl Rectangle` context. Note that we still need to use the `&` before `self`, just as we did in `&Rectangle`. Methods can take ownership of `self`, borrow `self` immutably as we have done here, or borrow `self` mutably, just as they can any other parameter.

We have chosen `&self` here for the same reason we used `&Rectangle` in the function version. We do not want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we have called the method on as part of what the method does, we would use `&mut self` as the first parameter. Having a method that takes ownership of the instance by using just `self` as the first parameter is rare. This technique is usually used when the method transforms `self` into something else ans you want to prevent caller from using original instance after transformation.

The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of `self` in every method's signature, is for organization. We have put all the things we can do with an instance of a type in one `impl` block rather than making future users of our code search for capabilities of `Rectangle` in various places in the library we provide.

> ### Where's the `->` Operator?
>In C and C++, two different operators are used for calling methods. You use `.` if you are calling method on the object directly and `->` if you are calling the method on a pointer to the object and need to dereference the pointer first. In other words, if `object` is a pointer, `object->something()` is similar to `(*object*).something()`.

>Rust does not have an equivalent to the `->` operator. Instead, Rust has a feature called _automatic referencing and dereferencing_. Calling methods is one of the few places in Rust that has this behavior.

>Here is how it works: When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*`, so `object` matches the signature of the method. In other words, the following are the same:

>```
>p1.distance(&p2);
>(&p1).distance(&p2);
>```

>The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver —the type of `self`—. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

### Methods With More Parameters
Let's practice using methods by implementing a second method on the `Rectangle` struct. This time, we want an instance of `Rectangle` to take another instance of `Rectangle` and return `true` if the second `Rectangle` can fit completely within `self`. Otherwise it should return `false`. That is, we want to be able to write the program shown in the next snippet, once we have defined the `can_hold` method.

```rust
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

And the expected output would look like the following, because both dimensions of `rect2` are smaller than the dimensions of `rect1` but `rect3` is wider that `rect1`.

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle` block. The method name will be `can_hold`, and it will take an immutable borrow of another `Rectangle` as a parameter. We can tell what the type of the parameter will be looking at the code that calls the method: `rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to `rect`, an instance of `Rectangle`. This makes sense because we only need to read `rect2`, and we want `main` to retain ownership of `rect2` so we can use it again after calling the `can_hold` method. The return value of `can_hold` will be a Boolean, and the implementation will check whether the width and height of `self` are both greater than the width and height of the other `Rectangle`, respectively. Let's add the new `can_hold` method to the `impl` block.

```rust

#![allow(unused_variables)]
fn main() {
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }

      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
}

```

When we run this code with the `main` function, we will get our desired output. Methods can take multiple parameters that we add to the signature after the `self` parameter, and those parameter work just like parameters in functions.

### Associated Functions
Another useful feature of `impl` blocks is that we are allowed to define functions within `impl` blocks that do not takes `self` as a parameter. These are called _associated functions_ because there are associated with the struct. They are still functions, not methods, because they do not have an instance of the struct to work with. You have already used the `String:from` associated function.

Associated functions are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square `Rectangle` rather than having to specify the same value twice.

```rust

#![allow(unused_variables)]
fn main() {
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn square(size: u32) -> Rectangle {
          Rectangle { width: size, height: size }
      }
  }
}
```

To call this associated, we use the `::` syntax with the struct name. `let sq = Rectangle::square3();` is an example. This function is namespaced by the struct. The `::` syntax is used for both associated functions and namespaces created by modules.

### Multiple `impl` Blocks
Each struct is allowed to have `impl` blocks. For example, the last code is equivalent to the code below, which has each method in its own `impl` block.

```rust

#![allow(unused_variables)]
fn main() {
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }

  impl Rectangle {
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
}
```
There is no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax. We will see a case in which multiple `impl` blocks are useful in chapter 10, where we discuss generic types and traits.

## Summary
Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. Methods let you specify the behavior that instances of your structs have, and associated functions let you namespace functionality that is particular to your struct without having an instance available.

But structs are no the only way you can create custom types. Let's turn Rust's enum feature to add another tool to your toolbox.
