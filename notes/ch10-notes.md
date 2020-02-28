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

### In Function Definitions
### In Structs Definitions
### In Enum Definitions
### In Method Definitions
### Performance of Code Using Generics


## Traits: Defining Shared Behavior
## Validating References with Lifetimes
