# Chapter 06: Enums and Pattern Matching
In this chapter, we will look _enumerations_. Enums allow you to define a type by enumerating its possible _variants_. First we will define and use enum to show how an enum can encode meaning along with data. Next, we will explore a particularly useful enum, called `Option`, which expresses that a value can be either something or nothing. Then we will look at how **pattern matching** in the `match` expression makes it easy to run different code for different values of an enum. Finally we will cover how the `if let` construct is another convenient and concise idiom available to you handle enums in your code.

Enums are a feature in many languages, but their capabilities differ in each language. Rust's enums are most similar to _algebraic data types_ in functional languages, such as OCaml and Haskell.

## Index
1. Defining an Enum
2. The Match Control Flow Operator
3. Concise Control Flow With If Let

## 1. Defining an Enum

### Enum Values

### The `Option` Enum and Its Advantages Over Null Values

## 2. The Match Control Flow Operator

### Patterns that Bind to Values

### Matching with `Option<T>`

### Matches are Exhaustive

### The `_` Placeholder

## 3. Concise Control Flow With If Let

## Summary
