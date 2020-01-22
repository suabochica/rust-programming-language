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

### Memory and Allocation

#### Ways Variables and Data Interact: Move

#### Ways Variables and Data Interact: Clone

#### Stack-Only Data: Copy

### Ownership and Functions

### Returns Values and Scope

## 2. References and Borrowing

## 3. The Slice Type
