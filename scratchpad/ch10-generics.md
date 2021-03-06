# Generic Types, Traits and Lifetimes
Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is _generics_.

> def: Generics are abstracts stand-ins for concrete types or other properties.

When we are writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

Similar to the way a function takes parameter with unknown values to run the same code on multiple concrete values, functions can take parameters of some generic type instead of a concrete type, like `i32` or `String`. In fact, we have already used generics in chapter 6 with `Option<T>~`, chapter 8 with `Vec<T>` and `HashMap<K, V>` and in chapter 9 with `Result<T, E>`. In this chapter, you will explore how to define your own types, functions, and methods with generics.

First we will review how to extract a function to reduce code duplication. Next we will use the same technique to make a generic function from two functions that differ only in the types of their parameters. We will also explain how to use generic types in struct and enum definitions.

Then you will learn how to use _traits_ to define behavior in a generic way. You can combine traits with generic types to constrain a generic type to only those types that have a particular behavior, as opposed to just any type.

Finally, we will discuss _lifetimes_, a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to borrow values in many situations while still enabling the compiler to check that the references are valid.

## Removing Duplication by Extracting a Function.

Before diving into generics syntax, let's first look at how to remove duplication that does not involve generic types by extracting a function. Then we will apply this technique to extract a generic function. In the same way that you recognize duplicated code to extract into a function, you will start to recognize duplicated code that can use generics.

Consider a short program that finds the largest number in a list.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0]
    
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    
    println!("The largest number is {}", largest);
}
```

This code stores a list of integers in the variable `number_list` and places the first number in the list in a variables named `largest`. Then it iterates through all the numbers in the list, and if the current number is greater than the number stored in `largest`, it replaces the number in that variable. However, if the current number is less than or equal to the largest number seen so far, the variables does not change, and the code moves on to the next number in the list. After considering all the numbers in the list, `largest` should hold the largest number, which in this case is 100.

To find the largest number in two different lists of numbers, we can duplicate the code before and use the same logic at two different places in the program, as shown below:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0]
    
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    
    println!("The largest number is {}", largest);
}

fn main() {
    let number_list = vec![34, 102, 85, 54, 600];
    let mut largest = number_list[0]
    
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    
    println!("The largest number is {}", largest);
}
```
Although this code works, duplicating code is tedious and error prone. We also have to update the code in multiple places when we want to change it.

To eliminate this duplication, we can create and abstraction by defining a function that operates on any list of integers given to it in a parameter. This solution makes our code clearer and lets us express the concept of finding the largest number in a list abstractly.

In the next snippet, we extracted the code that finds the largest number into a function named `largest`. Unlike the first code approach, which can find the largest number in only one particular list, this program can find the largest number in two different lists.

```rust
fn largest(list: &[i32]) -> {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 40, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![34, 102, 85, 54, 600];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

}
```
The `largest` function has a parameter called `list`, which represents any concrete slice of `i32` values that we might pass into the function. As a result, when we call the function, the code runs on the specific values that we pass in.

In sum, here are the steps we took to abstract our code:

1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.

Next, we will use these same steps with generics to reduce code duplication in different ways. In the same way the function body can operate on an abstract `list` instead of specific values, generics allow code to operate on abstract types.

For example, say we had two function: one that finds the largest item in a slice of `i32` values and one that finds the largest item in a slice of `char` values. How would we eliminate that duplication? Let's find out.

## Index
1. Generic Data Types
2. Traits: Defining Shared Behavior
3. Validating References with Lifetimes

## 1. Generic Data Types
We can use generics to create definitions for items like functions signatures or structs, which w can then use with many different concrete data types. Let's first look at how to define functions, structs, enums, and methods using generics. Then we will discuss how generics affect code performance.

### In Function Definitions
When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

Continuing with our `largest` function, code below shows two functions that both find the largest value in a slice.

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn largest(list: &[char]) -> char {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 40, 25, 100, 65];
    let result = largest_i32(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);
}
```

The `largest_32` function is the one we extracted in our last version that find the largest `i32` in a slice. The `largest_char` function finds the largest `char` in a slice. The function bodies have the same code, so let's eliminate the duplication by introducing a generic type parameter in a single function.

To parameterize the types in the new function we will define, we need to name the type parameter, just as we do for the value parameters to a function. You can use any identifier as a type parameter name. But we will use `T` because, by convention, parameter names in Rust are short, often just a letter, and Rust's type-naming convention is CamelCase. Short for "type" `T` is the default choice of most Rust programmers.

When we use a parameter in the body function, we have to declare the parameter name in the signature so the compiler knows what that name means. Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it. To define the generic `largest` function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list, like this:

```rust
fn largest<T>(list: &[T]) -> T {...}
```

We read this definition as the function `largest` is generic over some type `T`. This function has one parameter `list`, which is a slice of values of type `T`. The `largest` function will return a value of the same type `T`.

Next code version shows the combined version `largest`function definition using the generic data type in its signature. The listing also shows how we can call the function with either a slice of `i32` values or `char` values. Note that this code won't compile yet, but we will fix later.

```rust

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 40, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);
}
```

If we compile this code right now, we will get this error:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
   |
   5 |         if item > largest {
     |            ^^^^^^^^^^^^^^
       |
         = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

The note mentions `std::cmp::PartialOrd`, which is a _trait_. We will talk about traits in the next section. For now, this error states that the body of `largest` won't work for all possible types that `T` could be. Because we want to compare values of type `T` in the body, we can only use types whose values can be ordered. To enable comparisons, the standard library has the `str::cmp::PartialOrd` trait that you can implements on types. You will learn how to specify that a generic type has a particular trait in the _Traits as Parameters_  section, but let's first explore other ways of using generic type parameters.

### In Structs Definitions
We can also define structs to use generic type parameter in one or more fields using `<>` syntax. Next snippet shows how to define a `Point<T>` struct to hold `x` and `y` coordinate values of any type.


```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

The syntax of using generics in struct definitions is similar to that used in function definitions. First, we declare the name of the type parameter inside angle brackets just after the name of the struct. Then we can use the generic type in the struct definition where we would otherwise specify concrete data types.

Note that because we have used only one generic type to define `Point<T>`, this definition says that the `Point<T>` struct is generic over some type `T`, and the fields `x` and `y` are _both_ that same type, whatever that type may be. If we create an instance of a `Point<T>` that has values of different types, as we shown below, the code won't compile.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main () {
    let wont_work = Point { x: 5, y: 4.0 };
}
```
In this example, when we assign the integer value 5 to `x`, we let the compiler know that the generic type `T` will be an integer for this instance of `Point<Y>`. Then when we specify 4.0 for `y`, which we have defined to have the same type as `x`, we will get a type mismatch error like this.

```
error[E0308]: mismatched types
 --> src/main.rs:7:38
   |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found
floating-point number
  |
  = note: expected type `{integer}`
               found type `{float}`          
```

To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters. For example, in the next code version we can change the definition of `Point` to be generic over types `T` and `U` where x is of type `T` and `y` is of type `U`.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
  let both_integer = Point { x: 5, y: 10 };
  let both_float = Point { x: 1.5, y: 4.0 };
  let integer_float = Point { x: 5, y: 4.0 };
}
```

Now all the instance of `Point` shown are allowed. You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read. When you need lots of generic types in your code, it could indicate that your code need restructuring into smaller pieces.

### In Enum Definitions
As we did with structs, we can define enums to hold generic data types in their variants. Let's take another look at the `Option<T>` enum that the standard library provides.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This definitions should now make more sense to you. As you can see, `Option<T>` is an enum that is generic over type `T` and has two variants:

1. `Some` which holds one value of type `T`
2. `None` variant that does not hold any value

By using `Option<T>` enum, we can express the abstract concept of having an optional value, and because `Option<T>` is generic, we can use this abstraction no matter what the type of the optional value is.

Enums can use multiple generic types as well. The definition of the `Result`enum that we used in chapter 9 is one example:


```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, `T` and `E`, and has to variants:

1. `Ok` which holds one value of type `T`
2. `Err` which holds one value of type `E`

This definition makes it convenient to use the `Result` enum anywhere we have an operation that might succeed or fail. In fact, that is what we used to open a file in the respective example where `T` was filled in with the type `std::fs::File` when the file was opened successfully and `E` was filled in with the type `std::io::Error` when there were problems opening the file.

When you recognize this situation in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.

### In Method Definitions
We can implement methods on structs and enums and use generic types in their definitions too. Next example shows the `Point<T>` struct with a method named `x` implemented on it.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    
    println!("p.x = {}", p.x());
}
```

Here we have defined a method named `x` on `Point<T>` that returns a reference to the data in the field `x`.

Note that we have to declare `T` just after `impl` so we can use it to specify that we are implementing methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.

We could, for example implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type. Next code use the concrete type `f32`, meaning we do not declare any types after `impl`.

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powo(2)).sqrt()
    }
}
```

This code means the type `Point<f32>` will have a method named `distance_from_origin` and other instances of `Point<T>` where `T` is not of type `f32` will not have this method defined. The method measures how far our point is from the point at coordinates (0.0, 0.0) and uses mathematical operations that are available only for floating point types.

Generic types parameters in a struct definition are not always the same as those you use in that struc's method signatures. For example, the snippet below defines the method `mixup` on the `Point<T, U>` struct. The method takes another `Point` as a parameter, which might have different types from the `self` `Point` we are calling `mixup` on. The method creates a new `Point` instance with the `x` value from the `self` `Point` of the type `T` and the value `y` from the passed in `Point` of type `W`.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

In `main`, we have defined a `Point` that has an `i32` for `x` and a `f64` for `y`. The `p2` variable is a `Point` struct that has a string slice for `x` and a `char` for `y`. Calling `mixup` on `p1` with the argument `p2` gives us `p3`, which will have an `i32` for `x` came from `p1`. The `p3` variable will have a `char`for `y`, because `y` came from `p2`. The `println!` macro call will print `p3.x = 5, p3.y = c`.

The purpose of this example is to demonstrate a situation in which some generic parameters are with `impl` and some are declared with the method definition. Here, the generic parameters `T` and `U` are declared after `impl`, because they go with the struct definition. The generic parameters `V` and `W` are declared after `fn mixup`, because they are only relevant to the method.


### Performance of Code Using Generics
You might be wondering whether there is a runtime cost when you are using generic type parameters. The good news is that Rust implements generics in such a way that your code does not run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. _Monomorphization_ is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

In this process, the compiler does the opposite of the steps we used to create the generic function in some examples ago. The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

Let's look at how this works with an example that uses the standard library's `Option<T>` enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in `Option<T>` instances and identifies two kinds of `Option<T>`:

1. One is `i32`
2. Another one is `f64`

As such, it expands the generic definition of `Option<T>` into `Option_i32` and `Option_f64`, thereby replacing the generic definition with the specific ones.

The monomorphization version of the code looks like the following. The generic `Option<T>` is replaced with the specific definitions created by the compiler:

```rust
enum Option_i32 {
   Some(i32),
   None,
}
enum Option_f64 {
   Some(f64),
   None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Because Rust compile generics code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust's generics extremely efficient at runtime.

## 2. Traits: Defining Shared Behavior

A _trait_ tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.. We can use trait bounds or specify that a generic can be any type that has certain behavior.

> Note: Traits are similar to a feature often called _interfaces_ in other languages, although with some differences.

### Defining a Trait
A type's behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

For example, let's say we have multiple structs that hold various kinds and amount of text. A `NewsArticle` struct that holds a news story field in a particular location and a `Tweet` that can have a most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or reply to another tweet.

We want to make a media aggregator library that can display summaries of data that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we need to request that summary by calling a `summarize` method on an instance. Next code shows the definition of a `Summary` trait that expresses this behavior.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Here, we declare a trait using the `trait` keyword and then the trait's name, which is `Summary` in this case. Inside the curly brackets, we declare the method signature that describe the behaviors of the types that implement this trait, which in this case is `summarize(&self) -> String`.

After the method signature, instead of providing an implementation within curly brackets, we use a semicolon. Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

A trait can have multiple methods in its body. The method signatures are listed one per line a each ends in a semicolon.

### Implementing a Trait on a Type
Now that we have defined the desired behavior using the `Summary` trait, we can implement it on the types in our media aggregator. Next code shows an implementation of the `Summary` trait on the `NewsArticle` struct that uses the headline, the author, and the location to create the return value of `summarize`. For the `Tweet` struct, we define `summarize` as the username followed by the entire text of the tweet, assuming that tweet content is already limited to 280 characters.

```rust
pub struct NewsArticle {
    pub headline: String;
    pub location: String;
    pub author: String;
    pub content: String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format("{}: {})", self.username, self.content)
    }
}
```
Implementing a trait on a type is similar to implementing regular methods. The difference is that after `impl`, we put the trait name that we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for. Within the `impl` block, we put the method signatures that the trait definition has defined. Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

After implementing the trait, we can call the method on instances of `NewsArticle` and `Tweet` in the same way we call regular methods.

```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    contemt: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

This code prints `1 new tweet: horse_ebooks: of course , as you probably already know, people`.

Note that because we defined the `Summary` trait an the `NewsArticle` and `Tweet` types in the same `lib.rs`, they are all in the same scope. Let's say this `lib.rs` is for a crate we have called `aggregator` and someone else wants to use our crate's functionality to implement the `Summary` trait on a struct defined within their library's scope. They would need to bring the trait into their scope first. They would do so by specifying `use aggregator::Summary;`, which then would enable them to implement `Summary` for their type. The `Summary` trait would also need to be a public trait for another crate to implement it, which it is because we put the `pub` keyword before `trait`.

One restriction to note with trait implementation is that we can implement a trait on the type only if either the trait or the type is local to our crate. For example, we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our `aggregator` crate functionality, because the type `Tweet` is local to our `aggregator` crate. We can also implement `Summary` on `Vec<T>` in our aggregator crate, because the trait `Summary` is local to our `aggregator` crate.

But we cannot implement external traits on external types. For example, we cannot implement the `Display` trait on `Vec<T>` within our `aggregator` crate, because `Display` and `Vec<T>` are defined in the standard library and are not local to our `aggregator` crate. This restriction is part of a property of program called _coherence_, and more specifically the _orphan rule_, so named because the parent type is not present. This rules ensure that other people's code cannot break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust would not know which implementation to use.

### Default Implementations

Sometimes it is useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all method on every type. Then, as we implement the trait on a particular type, we can keep or override each method's default behavior.

Next code shows how to specify a default string for the `summarize` method of the `Summary` trait instead of only defining the method signature.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
To use a default implementation to summarize instances of `NewsArticle` instead of defining a custom implementation, we specify an empty `impl` block with `impl Summary for NewsArticle {}`.

Even though we are no longer defining the `summarize` method on `NewsArticle` directly, we have provided a default implementation and specified that `NewsArticle` implements the `Summary` trait. As a result, we can still call the `summarize` method on an instance of `NewsArticle`, like this:

```rust
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburhg Penguins once again are the best"),

};

println!("New article available! {}", article.summarize());
```

This code prints `New article available! (Read more..)`.

Creating default implementation for summarize does not require us to change anything about the implementation of Summary on Tweet. The reason is that the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that does not have a default implementation.

Default implementation can call other methods in the same trait, even if those other methods do not have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it. For example, we could define the `Summary` trait to have a `summarize_author` method whose implementation is required, and the define a `summarize` method that has a default implementation that calls the `summarize_author` method:

```rust

#![allow(unused_variables)]
fn main() {
pub trait Summary {
    fn summarize_author(&self) -> String;
    
        fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
                    
        }
        
}
}
```

To use this version of `Summary`, we only need to define `summarize_author` when we implement the trait on a type:

```rust
impl Summary for Tweet {
    summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

After we define `summarize_author`, we can call `summarize` on instances of the `Tweet` struct, and the default implementation of `summarize` will call the definition of `summarize_author` that we have provided. Because we have implemented `summarize_author`, the `Summary` trait has given us the behavior of the `summarize` method without requiring us to write any more code.

```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

This code prints `1 new tweet: (Read more form @horse_ebooks...)`.

Note that is not possible to call the default implementation from an overriding implementation of the same method.

### Traits as Parameters
Now that you know how to define and implement traits, we can explore how to use traits to define functions that accept many different types.

Before, we implemented the `Summary` trait on the `NewsArticle` and `Tweet` types. We can define a `notify` function that calls the `summarize` method on its item parameter, which is of some type that implements the `Summary` trait. To do this, we can use the `impl Trait` syntax like this:

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
    
}
```

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of `notify`, we can call any methods on `item` that come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the function with any other type, such as a `String` or an `i32`, won't compile because those types do not implement `Summary`.

#### Trait Bound Syntax
The `impl` Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound; it looks like this:


```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
    
}
```

This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement `Summary`. Using the impl Trait syntax looks like this:


```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {
```

If we wanted this function to allow `item1` and `item2` to have different types, using impl Trait would be appropriate (as long as both types implement `Summary`). If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:


```rust
pub fn notify<T: Summary>(item1: T, item2: T) {
```

The generic type T specified as the type of the `item1` and `item2` parameters constrains the function such that the concrete type of the value passed as an argument for `item1` and `item2` must be the same.

#### Specifying Multiple Trait Bounds
We can also specify more than one trait bound. Say we wanted `notify` to use display formatting on `item` as well as the `summarize` method: we specify in the `notify` definition that `item` must implement both `Display` and `Summary`. We can do so using the `+` syntax:


```rust
pub fn notify(item: impl Summary + Display) {
```

The `+` syntax is also valid with trait bounds on generic types:


```rust
pub fn notify<T: Summary + Display>(item: T) {
```

With the two trait bounds specified, the body of `notify` can call `summarize` and use `{}` to format `item`.

#### Clearer Trait Bounds
Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature. So instead of writing this:


```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

we can use a `where` clause, like this:

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
    {}
```
              
This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.

### Returning Types that Implement Traits
We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
                                        
    }
}
```

By using `impl Summary` for the return type, we specify that the `returns_summarizable `function returns some type that implements the Summary trait without naming the concrete type. In this case, returns `summarizable` returns a `Tweet`, but the code calling this function doesn’t know that.

The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13. Closures and iterators create types that only the compiler knows or types that are very long to specify. The `impl Trait` syntax lets you concisely specify that a function returns some type that implements the `Iterator` trait without needing to write out a very long type.

However, you can only use `impl Trait` if you’re returning a single type. For example, this code that returns either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary` wouldn’t work:

```rust
This code does not compile!
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
      NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
      }
    } else {
      Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
      }
    }
}
```

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler. We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.

### Fixing the `largest` Function
Now that you know how to specify the behavior you want to use using the generic type parameter’s bounds, let’s return to Listing 10-5 to fix the definition of the largest function that uses a generic type parameter! Last time we tried to run that code, we received this error:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
   ||
   5 |         if item > largest {
     | ^^^^^^^^^^^^^^ |        |
     |                |        |
     |                | #ERROR |
     |                |        |
     #+TBLFM: $2=note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
}||
```

In the body of largest we wanted to compare two values of type T using the greater than (>) operator. Because that operator is defined as a default method on the standard library trait std::cmp::PartialOrd, we need to specify PartialOrd in the trait bounds for T so the largest function can work on slices of any type that we can compare. We don’t need to bring PartialOrd into scope because it’s in the prelude. Change the signature of largest to look like this:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {}
```

This time when we compile the code, we get a different set of errors:

```
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
   ||
   2 |     let mut largest = list[0];
   |
```
The key line in this error is `cannot move out of type [T], a non-copy slice`. With our non-generic versions of the largest function, we were only trying to find the largest `i32` or `char`. As discussed in the “Stack-Only Data: Copy” section in Chapter 4, types like `i32` and char that have a known size can be stored on the stack, so they implement the `Copy` trait. But when we made the `largest` function generic, it became possible for the `list` parameter to have types in it that don’t implement the `Copy` trait. Consequently, we wouldn’t be able to move the value out of list[0] and into the `largest` variable, resulting in this error.

To call this code with only those types that implement the `Copy` trait, we can add `Copy` to the trait bounds of `T`! Next code shows the complete code of a generic `largest` function that will compile as long as the types of the values in the slice that we pass into the function implement the `PartialOrd` and `Copy` traits, like `i32` and `char` do.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
            if item > largest {
              largest = item;
            }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result);
            
    let char_list = vec!['y', 'm', 'a', 'q'];
                
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
If we don’t want to restrict the largest function to the types that implement the Copy trait, we could specify that T has the trait bound `Clone` instead of `Copy`. Then we could clone each value in the slice when we want the `largest` function to have ownership. Using the `clone` function means we’re potentially making more heap allocations in the case of types that own heap data like `String`, and heap allocations can be slow if we’re working with large amounts of data.

Another way we could implement `largest` is for the function to return a reference to a `T` value in the slice. If we change the return type to &T instead of `T`, thereby changing the body of the function to return a reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could avoid heap allocations. Try implementing these alternate solutions on your own

### Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. For example, the type Pair<T> in Listing 10-16 always implements the `new` function. But Pair<T> only implements the cmp_display method if its inner type `T` implements the `PartialOrd` trait that enables comparison and the `Display` trait that enables printing.

```rust

#![allow(unused_variables)]
fn main() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);

            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket _implementations_ and are extensively used in the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code:

```rust
impl<T: Display> ToString for T {}
```
Because the standard library has this blanket implementation, we can call the to_string method defined by the `ToString` trait on any type that implements the `Display` trait. For example, we can turn integers into their corresponding `String` values like this because integers implement `Display`:

```rust
let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the “Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t implement the type which defines the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

Another kind of generic that we’ve already been using is called _lifetimes_. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be. Let’s look at how lifetimes do that.

## 3. Validating References with Lifetimes

One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that every reference in Rust has a _lifetime_, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature. Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so you can become familiar with the concepts.

### Preventing Dangling References with Lifetimes
The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference. Consider the next program, which has an outer scope and an inner scope.

```rust
{
    let r;
    
    {
        let x = 5;
        r = &x;
    }
    
    println!("r: {}", r);
}
```
The outer scope declares a variable named `r` with no initial value, and the inner scope declares a variable named `x` with the initial value of 5. Inside the inner scope, we attempt to set the value of `r` as a reference to `x`. Then the inner scope ends, and we attempt to print the value in `r`. This code won’t compile because the value `r` is referring to has gone out of scope before we try to use it. Here is the error message:

```
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:5
     ||
     6  |         r = &x;
     ||
```
The variable `x` doesn’t “live long enough.” The reason is that `x` will be out of scope when the inner scope ends on line 7. But r is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to do with `r` wouldn’t work correctly. So how does Rust determine that this code is invalid? It uses a borrow checker.

### The Borrow Checker
The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid. Next snippet shows the same code as Listing 10-17 but with annotations showing the lifetimes of the variables.

```rust
{
    let r;                // ---------|-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -|-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with `'b`. As you can see, the inner `'b` block is much smaller than the outer `'a` lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with a lifetime of `'b`. The program is rejected because `'b` is shorter than `'a`: the subject of the reference doesn’t live as long as the reference.

Next code fixes our issue, so it does not have a dangling reference and compiles without any errors.

```rust

#![allow(unused_variables)]
fn main() {
{
    let x = 5;            // ----------|-- 'b
                          //           |
    let r = &x;           // --|-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure references will always be valid, let’s explore generic lifetimes of parameters and return values in the context of functions.

### Generic Lifetimes in Functions
Let’s write a function that returns the longer of two string slices. This function will take two string slices and return a string slice. After we’ve implemented the longest function, the next code should print The longest string is abcd.

```rust
fn main() {
    let string1 = String::from("abdc");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest strinf is {}", result);
    
}
```

Note that we want the function to take string slices, which are references, because we don’t want the `longest` function to take ownership of its parameters. We want to allow the function to accept slices of a String (the type stored in the variable `string1`) as well as string literals (which is what variable `string2` contains).

Refer to the “String Slices as Parameters” section in Chapter 4 for more discussion about why the parameters we use in before are the ones we want.

If we try to implement the longest function as shown in below, it won’t compile.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() ? y.len() {
        x
    } else {
        y
    }
}
```

Instead, we get the following error that talks about lifetimes:

```
error[E0106]: missing lifetime specifier
```

The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to `x` or `y`. Actually, we don’t know either, because the if block in the body of this function returns a reference to `x` and the else block returns a reference to `y`!

When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the `if` case or the else case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did before to determine whether the reference we return will always be valid. The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of `x` and `y` relate to the lifetime of the return value. To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

### Lifetime Annotation Syntax
Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types. Most people use the name `'a`. We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference’s type.

Here are some examples: a reference to an `i32` without a lifetime parameter, a reference to an `i32` that has a lifetime parameter named `'a`, and a mutable reference to an `i32` that also has the lifetime `'a`.)

```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. For example, let’s say we have a function with the parameter first that is a reference to an `i32` with lifetime `'a`. The function also has another parameter named second that is another reference to an `i32` that also has the lifetime `'a`. The lifetime annotations indicate that the references `first` and `second` must both live as long as that generic lifetime.

### Lifetime Annotations in Function Signatures
Now let’s examine lifetime annotations in the context of the `longest` function. As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list. The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime. We’ll name the lifetime `'a` and then add it to each reference, as shown next.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() ? y.len() {
        x
    } else {
        y
    }
}
```
This code should compile and produce the result we want when we use it.

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`. In practice, it means that the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the references passed in. These constraints are what we want Rust to enforce. Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the `longest` function doesn’t need to know exactly how long `x` and `y` will live, only that some scope can be substituted for `'a` that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words, the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes. Snippet below is a straightforward example.

```rust
fn main() {
    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        
        println!("The longest string is {}", result);
    }
}
```

In this example, `string1` is valid until the end of the outer scope, `string2` is valid until the end of the inner scope, and `result` references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves of this code; it will compile and print `The longest string is long string is long`.

Next, let’s try an example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments. We’ll move the declaration of the result variable outside the inner scope but leave the assignment of the value to the result variable inside the scope with `string2`. Then we’ll move the `println`! that uses result outside the inner scope, after the inner scope has ended. 


```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
When we try to compile this code, we’ll get this error:

```
error[E0597]: `string2` does not live long enough
```

The error shows that for `result` to be valid for the println! statement, `string2` would need to be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter `'a`.

As humans, we can look at this code and see that `string1` is longer than `string2` and therefore result will contain a reference to `string1`. Because `string1` has not gone out of scope yet, a reference to `string1` will still be valid for the println! statement. However, the compiler can’t see that the reference is valid in this case. We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. Therefore, the borrow checker disallows the last code as possibly having an invalid reference.

Try designing more experiments that vary the values and lifetimes of the references passed in to the longest function and how the returned reference is used. Make hypotheses about whether or not your experiments will pass the borrow checker before you compile; then check to see if you’re right!

### Thinking in Terms of Lifetimes
The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the `longest` function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter. The following code will compile:

```rust

#![allow(unused_variables)]
fn main() {
  fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x   
  }
}
```

In this example, we’ve specified a lifetime parameter `'a` for the parameter `x` and the return type, but not for the parameter `y`, because the lifetime of `y` does not have any relationship with the lifetime of `x` or the return value.

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function. Consider this attempted implementation of the longest function that won’t compile:

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
        result.as_str()
        
}
```

Here, even though we’ve specified a lifetime parameter `'a` for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all. Here is the error message we get:

```
error[E0597]: `result` does not live long enough
note: borrowed value must be valid for the lifetime 'a as defined on the
function body at 1:1...'
```

The problem is that `result` goes out of scope and gets cleaned up at the end of the `longest` function. We’re also trying to return a reference to `result` from the function. There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference. In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

### Lifetime Annotations in Struct Definitions
o far, we’ve only defined structs to hold owned types. It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition. The next code has a struct named `ImportantExcerpt` that holds a string slice.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

This `struct` has one field, `part`, that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.

The `main` function here creates an instance of the `ImportantExcerpt` struct that holds a reference to the first sentence of the String owned by the variable `novel`. The data in novel exists before the `ImportantExcerpt` instance is created. In addition, `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

### Lifetime Elision
You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references. However, in Chapter 4 we had a function that compiled without lifetime annotations.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. At that time, the function signature would have been written like this:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {...}
```
After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the _lifetime elision rules_. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. In this case, instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations that specify how the references relate to each other.

Lifetimes on function or method parameters are called _input lifetimes_, and lifetimes on return values are called _output lifetimes_.

The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations

1. The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
2. The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut` self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

Let’s pretend we’re the compiler. We’ll apply these rules to figure out what the lifetimes of the references in the signature of the `first_word`. The signature starts without any lifetimes associated with the references:

```rust
fn first_word(s: &str) -> &str {...}
```

Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime. We’ll call it 'a as usual, so now the signature is this:

```rust
fn first_word<'a>(s: &'a str) -> &str {...}
```

The second rule applies because there is exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {...}
```

Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

Because the third rule really only applies in method signatures, we’ll look at lifetimes in that context next to see why the third rule means we don’t have to annotate lifetimes in method signatures very often.

### Lifetime Annotations in Method Definitions
When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters shown before. Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method parameters and return values.

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

In method signatures inside the `impl` block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures. Let’s look at some examples using the struct named `ImportantExcerpt` that we defined before.

First, we’ll use a method named level whose only parameter is a reference to self and whose return value is an `i32`, which is not a reference to anything:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after `impl` and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.

Here is an example where the third lifetime elision rule applies:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &`self` and `announcement` their own lifetimes. Then, because one of the parameters is &`self`, the return type gets the lifetime of &`self`, and all lifetimes have been accounted for.

### The Static Lifetime
One special lifetime we need to discuss is `'static`, which means that this reference can live for the entire duration of the program. All string literals have the `'static` lifetime, which we can annotate as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.

You might see suggestions to use the `'static` lifetime in error messages. But before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not. You might consider whether you want it to live that long, even if it could. Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the `'static` lifetime.


### Generic Type Parameters, Trait Bounds, and Lifetimes Together
Let's briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function:

```rust
use std::fmt::Display

fn longest_with_an_announcement<'a, T>(x: &'a str, ann: T) -> &'a str
    where T: Display
    
{
    println!("Announcement! {}", ann);
    if x.leng() ? y.len() {
        x
    } else {
        y
    }
}
```

This is the `longest` function that returns the longer of two string slices. But now it has an extra parameter named `ann` of the generic type `T`, which can be filled in by any type that implements the `Display` trait as specified by the where clause. This extra parameter will be printed before the function compares the lengths of the string slices, which is why the `Display` trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name.'

## Summary

We covered a lot in this chapter! Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations. Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!

Believe it or not, there is much more to learn on the topics we discussed in this chapter: Chapter 17 discusses trait objects, which are another way to use traits. Chapter 19 covers more complex scenarios involving lifetime annotations as well as some advanced type system features. But next, you’ll learn how to write tests in Rust so you can make sure your code is working the way it should.
