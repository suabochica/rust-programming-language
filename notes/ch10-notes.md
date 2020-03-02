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

## Generic Data Types
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

## Traits: Defining Shared Behavior
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
### Traits as Parameters
#### Trait Bound Syntax
#### Specifying Multiple Trait Bounds
#### Clearer Trait Bounds

### Returning Types that Implement Traits
### Fixing the `largest` Function
### Using Trait Bounds to Conditionally Implement Methods


## Validating References with Lifetimes
