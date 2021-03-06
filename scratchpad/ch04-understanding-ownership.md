# Chapter 4: Understanding Ownership
Ownership in Rust is mos unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it is important to understand how ownership works in Rust. In this chapter, we will talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

## Index
1. What is Ownership?
2. References and Borrowing
3. The Slice Type

## 1. What is Ownership?
Rust's central feature is _ownership_. Although the feature is straightforward to explain, it has deep implications for the rest of the language.

All programs have to manages the way they use a computer's memory while running.

Some languages have _garbage collection_ that constantly looks for no longer used memory as the program runs. In other languages, the programmer must explicitly allocate and free the memory.

Rust uses a third approach, memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership feature slow down your program while it is running.

Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that mor experienced you become with rust and the rules of the ownership system, the more you will be able to naturally develop code that is safe and efficient. Keep at it!.

When you understand ownership, you will have a solid foundation for understanding the feature the make Rust unique. So, we will review some examples for understanding the concept with very common data structures, but before, we should recap the stack and the heap concepts.

### The Stack and the Heap
In many programming language you don't have to think about the stack and the heap very often. However, in systems programming language like Rust, whether a values is on the stack or the heap has more of an effect on how the language behaves ans why you have to make certain decisions.

Both, are parts of memory that are available to your code to use at runtime, but they are structured in different ways:

The stack store values in order it gets them and removes the values in the opposite order. This is referred as LIFO (Last In, First Out). Thick of a stack of plates, when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom would not work as well. Adding data is called _pushing onto the stack_, and removing data is called _popping off the stack_. **All data on the stack must have a know, fixed size**.

Data with an unknown size at compile time or size that might change must be stored on the heap instead. The heap is less organized. When you put data on the heap, you request a certain amount of space. The operating system finds an empty spot in the heap that is big enough, ,arks it as being in use, and returns a __pointer_, which is the address of that location. This process is called **allocating on the heap**. Pushing values onto the stack is not considered allocating. Because the pointer is a know, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Think of being seated  at a restaurant. When you enter, you state the number of people inn your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, the can ask where you have been seated to find you.

Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data. That location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the operating system must first find a big enough space to hold the data and then perform bookkeeping to prepare the next allocation.

Accessing data in the heap is slower that accessing data in the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.

Continuing with the analogy, consider a server at a restaurant taking order from many tables. It is most efficient to get all the others at one tables before moving on the next table. By the same token, a processor can do its job better if it works on data that is close to other data (i.e. the stack), than farther away (i.e. the heap). Allocating a large amount of space on the heap can also take time.

When your code calls a function, the values passed into the function – probably including pointers to data on the heap –, and function's local variables get pushed onto the stack. When the function it is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you do not run out space are all problems that ownership addresses. Once you understand ownership, you won't need to think about the stack and the heap very often, but knowing that managing heap data is why ownership exists can help explain why it works the way it does.

### Ownership Rules
The ownership rules are three:

+ Each value in Rust has a variable that is called its _owner_.
+ There can only be one owner at a time.
+ When the owner goes out of the scope, the value will be dropped.

Please keep in mind these rules when we work through the example to illustrate them.

### Variable Scope
As a first example of ownership, we will look at the _scope_ of some variables. A scope is the range within a program for which an item is valid. Let's say we have a variable that looks like this:

```rust
#![allow(unused_variables)]
fn main() {
  let s = "hello";
}
```

The variable `s` refers to a string literal, where the value of the string is hardcoded into the text of out program. The variable is valid from the point at which it is declared until the end of the current scope. Below a snippet with the comments where `s` is valid.

```rust
#![allow(unused_variables)]
fn main() {
  {                      // s is not valid here, it’s not yet declared
      let s = "hello";   // s is valid from this point forward

    // do stuff with s
  }                      // this scope is now over, and s is no longer valid
}

```

In other words, there are two important points here:

+ When `s` comes into scope, it is valid
+ It remains valid until it goes out of scope

At this point, the relationship between scopes and when variable are valid is similar to that in other programming languages. Now we will build on top of this understanding by introducing the `String` type.

### The String Type
To illustrate the rules of ownership, we need a data type that is more complex than the ones we covered in the Data Types section of chapter 3. The types covered previously are all stored on the stack and pooped off the stack when their scope is over, but we want to look at data that is sotred on the heap and explore how Rust knows when to clean up that data.

We will use `String` as the example here and concentrate on the parts of `String` that relate to ownership. these aspects also apply to other complex data types, whether they are provided by the standard library or created by you.

We have already seen string literals, where a string value is hardcoded into our program. Sting literals are convenient, but they are not suitable for every situation in which we may want to use text. One reason is that they are immutable. Another is that not every string value can be known when we write our code. For example, what if we want to take user input and store it? For these situation, Rust has second a second string type, `String`. This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a `String` from a string literal using the `from` function, like so.

```rust
#![allow(unused_variables)]
fn main() {
  let s = String::from("hello");
}
```

The double colon `::` is an operator that allows us to namespace this particular `from` function under the `String` type rather that using some sort of name like `string_from`. We will discuss this syntax in the _Method Syntax_ section of the next chapter and when we talk about namespacing with modules in _Paths for Referring to an item in the Module Tree_ in chapter 7.

This kind of string can be mutated.

```rust
#![allow(unused_variables)]
fn main() {
  let mut s = String::from("hello");

  s.push_str(", world!"); // push_str() appends a literal to a String

  println!("{}", s); // This will print `hello, world!`
}
```

So, why can `String` be mutated but literals cannot? The difference is how these two types deal with memory.

### Memory and Allocation
In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But, these properties only come from the string literal's immutability. Unfortunately, we cannot put a blob of memory into the binary for each piece of text whose size is unknown at compile time an whose size might change while running the program.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means.

+ The memory must be requested from the operating system at runtime.
+ We need a way of returning this memory to the operating system when we are done with our `String`.

That first part is done by us. When we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part it is different. In languages with garbage collector (GC), it keeps track and cleans up memory that is not being used anymore, and we do not need to thing about it. Without GC, it is our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we will waste memory. If we do it too early, we will have an invalid variable. If we do it twice, that is a bug too. We need to pair exactly one `allocate` withy exactly one `free`.

Rust takes a different path. The memory is automatically returned once the variable that owns it goes out of scope. Here is a version of our scope example is usign a `String` instead of a string literal.

```rust

#![allow(unused_variables)]
fn main() {
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
}

```

There is a natural point at which we can return the memory our `String` needs to the operating system. When `s` goes out of the scope. When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it is where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

> **Note:** In C++, this pattern of dellocating resources at the end of item's lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_. The `drop` function in rust will be familiar to you if you have used RAII patterns.

This pattern has a profound impact on the way Rust code is written. It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we have allocated on the heap. Let's explore some of those situations.

#### Ways Variables and Data Interact: Move
Multiple variables can interact with the same data in different ways in Rust. Let's look at an example using an integer.

```rust

#![allow(unused_variables)]
fn main() {
  let x = 5;
  let y = x;
}
```

We can probably guess what this is doing, bind the value `5` to `x`, then make a copy of the value in `x` and bind it to `y`. We now have two variables, `x` and `y`, and both equal to `5`. This is indeed what is happening, because integers are simple values with a know, fixed size, and these two `5` values are pushed onto the stack.

Now, let's look at the `String` version.

```rust

#![allow(unused_variables)]
fn main() {
  let s1 = String::from("hello");
  let s2 = s1;
}

```

This looks very similar to the previous code, so we might assume that the way it works would be the same. That is, the second line would make a copy of the value `s1` and bind it to `s2`. But this is not quite what happens.

Take a look to the next figure to see what is happening to `String` under the covers. A `String` is made up of three parts, show on the left, a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

![image](../assets/01_memory_representation_string.png)

The length is how much memory, in bytes, the contents of the `String` is currently using. The capacity is the total amount of memory, in bytes, that the `String` has received from the operating system. The difference between length and capacity matters, but not in this context, so for now, it is fine to ignore the capacity.

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. In other words, the data representation in memory looks like this.

![image](../assets/02_representation_in_memory_s1_s2.png)

The representation does not look like the next image, which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation `s1 = s2` could be very expensive in terms of runtime performance if the data on the heap were large.

![image](../assets/03_another_s1_s2_possibility.png)

Earlier, we said that when a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable. But the first image of representation of memory shows both data pointers pointing to the same location. This is a problem, when `s1` and `s2` go out of scope, they will both try to free the same memory. This is known as _double free_ error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, there is one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory. Rust `s1` to no longer be valid and, therefore, Rust does not need to free anything when `s1` goes out of scope. Check out what happens when you try to use `s1` after `s2` is created. It won't work.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

You will get an error like this because Rust prevents you form using the invalidated reference.

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait

```

If you have heard the therms _shallow copy_ and _deep copy_ while working with other languages, the concept of copying the pointer, length and capacity without copying the data probably sound like making a shallow copy. But because Rust also invalidates the first variables, instead of being called a shallow copy, it is known as a _move_. In this example, we would say that `s1` was moved into `s2`. So what actually happens is showing next.

![image](../assets/04_move_representation.png)

That solves our problem! with only `s2` valid, when it goes out of scope, it alone will free the memory and we are done.

In addition, there is a design choice that is implied by this. Rust will never automatically create _deep_ copies of your data. Therefore, any _automatic_ copying can be assumed to be inexpensive in terms of performance.

#### Ways Variables and Data Interact: Clone
If we do want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`. It is probably that you have seen this method in other programming languages. Below, an example of the `clone` method in action.

```rust

#![allow(unused_variables)]
fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
}

```

This works just fine and explicitly produces the behavior shown in the last image, where the heap data does get copied.

 When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. It is a visual indicator that something different is going up.

#### Stack-Only Data: Copy
There is another wrinkle we have not talked about yet. This code using integers, part of which shown before, works and is valid.

```rust

#![allow(unused_variables)]
fn main() {
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
}
```

But this code seems to contradict what we just learned. We do not have a call to `clone`, but `x` is still valid and was not moved into `y`.

The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there is no reason we would want to prevent `x` from being valid after we create the variable `y`. In other words, there is no difference between deep and shallow copying here, so calling `clone` would not do anything different from the usual shallow copying and we can leave out.

Rust has a special annotation called `Copy` trait that we can place on types like integers that are stored on the stack. If a type has the `Copy` trait, an older variable is still usable after assignment. Rust won't let us annotate a type with the `Copy` trait if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the values goes out of scope and we add the `Copy` annotation to that type, we will get a compile-time error.

So what type are `Copy` You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values van be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`.

+ All the integers types, such as `u32`.
+ The Boolean type, `bool`, with values `true` and `false`.
+ All the floating point types, such as `f64`.
+ The character type, `char`.
+ Tuples, if they only contain types that are also `Copy`.

### Ownership and Functions
The semantics for passing value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. The next snippet has an example with some annotations showing where variables got into and out of the scope.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a compile-time error.
These static check protect us from mistakes. Try adding code to `main` that uses `s` and `x` to see where you can use them and where the ownership rules prevent you from doing so.

### Returns Values and Scope
Returning values can also transfer ownership. The next code have similar annotations that we put in the functions ownership example.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
The ownership of a variable follows the same pattern every time. Assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It is quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

It is possible to return multiple values using a tuple, as shown below.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

```

But this is too much ceremony and a lot of work for concept that should be common. Luckily for us, Rust has a feature for this concept, called _references_.

## 2. References and Borrowing
The issue with the tuple code was that we have to return the `String` to the calling function so we can still use the `String` after the call to `calculate_length`, because the `String` was moved into `calculate_length`.

Here is how you would define and use a `calculate_length` function that has a reference to an object as a parameter instead of taking ownership of the value.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

First, notice that all the tuple code in the variable declaration and the function return is value is gone. Second, note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`.

These ampersands are _references_, and they allow you to refer to some value without taking ownership of it. Let's see the next diagram.

![image](../assets/05_reference_pointing.png)

> **Note:** The opposite of referencing by using `&` is _dereferencing_, which is accomplished with the dereference operator, `*`.

Let's take a closer look at the function call here:

```rust
#![allow(unused_variables)]
fn main() {
fn calculate_length(s: &String) -> usize {
    s.len()
}
let s1 = String::from("hello");

let len = calculate_length(&s1);
}
```

The `&s1` syntax lets us create a reference that _refers_ to the value of `s1` but does not own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

Likewise, the signature of the function use `&` to indicate that the type of the parameter is `s` is a reference. Let's add some explanatory annotations.

```rust
#![allow(unused_variables)]
fn main() {
  fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
  } // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
}
```

The scope in which the variable `s` is valid is the same as any function parameter's scope, but we don't drop what the reference points to when it goes out of scope because we don't have ownership. When functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership.

We call having references as function parameters _borrowing_. As in real life, if a person owns something, you can borrow it from them. When you are done, you have to give it back.

So what happens if we try to modify something we are borrowing? Try the code in the next snippet. Spoiler alert, it does not work!

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

```

Here is the error.

```
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

Just as variables are immutable by default, so are references. We are not allowed to modify something we have a reference to.

### Mutable References
We can fix the error in the code with the next small tweak.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First, we had to change `s` to be `but`. Then we had to create a mutable reference with `&mut s` and accept a mutable reference with `some_string: &mut String`.

But mutable references have one big restriction. **You can have only one mutable reference to a particular piece of data in a particular scope**. This code will fail.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Here is the error.

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```

This restriction allows for mutation but in a very controlled fashion. It is something that new Rustaceans struggle with, because most languages let you mutate whenever you would like.

The benefit of having this restriction is that Rust can prevent data races at compile time. A _data race_ is similar to a race condition and happens when these three behaviors occur.

+ Two or more pointers access the same data at the same time.
+ At least one of the pointers is being used to write to the data.
+ There is no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you are trying to track them down at runtime. Rust prevents this problem from happening because it won't even compile code with data races.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not _simultaneous_ ones.

```rust

#![allow(unused_variables)]
fn main() {
  let mut s = String::from("hello");

  {
    let r1 = &mut s;

  } // r1 goes out of scope here, so we can make a new reference with no problems.

  let r2 = &mut s;
}
```

A similar rule exists for combining mutable and immutable references. This code results in an error.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

```
Here is the error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here
```

We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don't expect the values to suddenly change out from under them. However, multiple immutable reference are okay because no one who is just reading the data has the ability to affect anyone else's reading of the data.

Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references occurs before the mutable reference is introduced.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last, which is before the mutable reference `r3` is created. These scopes don't overlap, so this code is allowed.

Even though borrowing errors may be frustrating at times, remember that it is the Rust compiler pointing out a potential bug early and showing you exactly where the problem is. Then you don't have to track down why your data is not what you thought it was.

### Dangling References
In languages with pointers, it is easy to erroneously create a _danling pointer_ a pointer that reference a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references. If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Lets try to create a dangling references, which Rust will prevent with a compile-time error.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here is the error:

```
error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

This error message refers to a feature we have not covered yet. **lifetimes**. We will discuss lifetimes in chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem.

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Let's take a closer look at exactly what is happening at each stage of out `dangle` code.

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid `String`. That is no good! Rust won't let us do this.

The solution here is to return the `String` directly.

```rust

#![allow(unused_variables)]
fn main() {
  fn no_dangle() -> String {
    let s = String::from("hello");

    s
  }
}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

### The Rules of References
Let's recap what we have discussed about references

+ At any given time, you can have _either_ one mutable reference _or_ any number of immutable references.
+ References must always be valid

## 3. The Slice Type

### String Slice
Another data type that does not have ownership is the _slice_. Slices let you reference a contiguous sequence of elements in a collections rather than the whole collection.

Here is a small programming problem: write a function that takes a string and returns the first word it finds in that string. If the function does not find a space in the string, the whole string must be one word, so the entire string should be returned.

Lets think about the signature of this function.

```rust
fn first_word(s: &String) -> ?
```

This function, `first_word`, has a `&String` as a parameter. We do not want ownership, so this is fine. But what should we return? We do not really have a way to talk about _part_ of a string. However, we could return the index of the end of the word. Let's try that.

```rust

#![allow(unused_variables)]
fn main() {
  fn first_word(s: &String) -> usize {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return i;
          }
      }

      s.len()
  }
}
```

Because we need to go through the `String` element by element and check whether a values is a space, we will convert our `String` to an array of bytes using the `as_bytes`  method. Next, we create an iterator over the array of bytes using the `iter` method. We will discuss iterators in more detail in chapter 13.

For now, know that `iter` is a method that returns each element in a collection that `enumerate` wraps the result of `iter` and returns each element as part of a tuple instead. The first element of the tuple returned from `enumerate` is the index, and the second element is a reference to the element. This is a bit more convenient that calculating the index ourselves.

Because the `enumerate` method returns a tuple, we can use patterns tor destructure that tuple, just like everywhere in Rust. So, in the `for` loop, we specify a pattern that has `i` for the index in the tuple and `&item` for the single byte in the tuple. Because we get a reference to the element from `iter().enumerate()` we use `&` in the pattern.

Inside the `for` loop, we search for the byte that represents the space by using the byte literal syntax. If we find a space, we return a position. Otherwise, we return the length of the string by using `s.len()`.

We now have a way to find out the index of the end of the first word in the string, but there is a problem. We are returning `usize` on its own, but it is only meaningful number in the context of the `&String`. In other words, because it is a separate value from the `String`, there is no guarantee that it will still be valid in the future. Consider the next program that is using the `firs_word` function and pay attention to the annotations.

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

This program compiles without any errors and would also do if we used `word` after calling `s.clear`(). Because `word` is not connected to the state of `s` at all, `word` still contains the value `5`. We could use that value 5 with the variable `s` to try to extract the first word out, but this would be a bug because the contents of `s` have changed since we save 5 in `word`.

Having to worry about the index in `word` getting out of sync with the data in `s` is tedious and error prone. Managing these indices is even more brittle if we write a `second_word` function. Its signature would have to look like this.

```rust
fn seconf_word(S: &String) -> (usize, usize) {}
```

Now we are tracking a starting and an ending index, and we have even more values that were calculated from data in a particular state but are not tied to that state at all. We now have three unrelated variables floating around that need to be kept in sync.

Luckily, Rust has a solution for this problem, string slice.

### String Slices
A _string slice_ is a reference to part of a `String`, and it looks like this:

```rust

#![allow(unused_variables)]
fn main() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
}
```

This is similar to taking a reference to the whole `String` but with the extra `[0..5]` bit. Rather than a reference to the entire `String`, it is a reference to a portion of the `String`.

We can create slices using a range within brackets by specifying `[starting_index..ending_index]`. Internally, the slice data structure stores the starting position and the length of the slice, which correspond to the `ending_index` minus `starting_index`. So in the case of `let word = &s[6..11];`, `world` would be a slice that contains a pointer to the 7th byte of `s` with a length value of 5. The next diagram illustrates this descriptions.

![image](../assets/06_slice_referring.png)

With Rust's `..` range syntax, if you want to start at the firs index (zero), you can drop the value before the two periods. In other words, these are equal:

```rust

#![allow(unused_variables)]
fn main() {
  let s = String::from("hello");

  let slice = &s[0..2];
  let slice = &s[..2];
}
```

By the same token, if your slice includes the last byte of the `String`, you can drop the trailing number. That means these are equal:

```rust

#![allow(unused_variables)]
fn main() {
  let s = String::from("hello");

  let len = s.len();

  let slice = &s[3..len];
  let slice = &s[3..];
}
```

You can also drop both values to take a slice of the entire string. So these are equal.

```rust

#![allow(unused_variables)]
fn main() {
  let s = String::from("hello");

  let len = s.len();

  let slice = &s[0..len];
  let slice = &s[..];
}
```

With all this information in mind, let's rewrite `first_word` to return a slice. The type that signifies "string slice" is written as `&str`:

```rust

#![allow(unused_variables)]
fn main() {
  fn first_word(s: &String) -> &str {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return &s[0..i];
          }
      }

      &s[..]
  }
}
```

We get the index for the end of the word by looking for the first occurrence of a space. When we find a space, we return a string slice using the start to string and the index of the space as the starting and ending indices.

Now when we call `first_world`, we get back a single values that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function.

```rust
fn second_word(s: &String) -> &str {}
```

We now have a straightforward API that is much harder to mess up, because the compiler will ensure the reference into `String` remain valid. Remember the bug when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but did not show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of `first_word` will throw a compile-time error:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

Here is the compile error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because `clear` needs to truncate the `String`, it needs to get a mutable reference. Rust disallow this, and compilation fails. Not only has Rust made our API easier to use, but is has also eliminated an entire class of errors at compile time.

#### String Literals are Slices
Recall that we talked about string literals, being stored inside the binary. Now that we now about the slices, we can properly understand string literals.

```rust

#![allow(unused_variables)]
fn main() {
  let s = "Hello, world!";
}
```
The type of `s` her is `&str`. It is a slice pointing to that specific point of the binary. This is also why string literals are immutable. `&str` is an immutable reference.

#### String Slices as Parameters
Knowing that you can take slices of literals and `String` values leads us to one more improvement on `first_word`, and that is its signature.

```rust
fn first_word(s: &String) -> &str {}
```

A more experienced Rustacean would write the next signature instead, because it allows us to use the same function on both `&String` values and `&str` values.

```rust
fn first_word(s: &str) -> &str {}
```

If we have a string slice, we can pass that directly. If we have a `String`, we can pass a slice of the entire `String`. Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality.

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

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices
String slices, as you might imagine, are specific to strings. But there is a more general slice type, too. Consider this array.

```rust
#![allow(unused_variables)]
fn main() {
  let a = [1, 2, 3, 4, 5];
}
```

Just as we might want to refer to a part of a string, we might want to refer to part of ans array. We would do so like this.

```rust
#![allow(unused_variables)]
fn main() {
  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];
}
```

This slice has the type `&[i32]`. It works the same way as string slices do, by storing a reference to the first element and a length. You will use this kind of slice for all sorts of other collections. We will discuss these collections in detail whe we talk about vectors in chapter 8.

### Summary
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you do not have to write and debug extra code to get this control.

Ownership affect how lots of other parts of Rust work, so we will talk about these concepts further throughout the rest of the book. Let's move on the chapter 5 and look at grouping pieces of data together in a `struct`.
