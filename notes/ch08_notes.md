# Chapter 08: Common Collections

Rust's standard library includes a number of very useful data structures called _collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuples, the data these collection point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing and appropriate one for your current situation is a skill you will develop over time. In this chapter we will discuss three collections that are used very often in Rust programs:

+ A _vector_ allows you to store a variable number of values next to each other.
+ A _string_ is a collection of characters. We have mentioned the `String` type previously, but in this chapter we will talk about it in depth.
+ A _hash map_ allows you to associate a value with a particular key. It is a particular implementation of the more general data structure called a _map_.

## Links
+ [Other kind of collections](https://doc.rust-lang.org/std/collections/index.html)

Time to discuss how to create and update vectors, strings and hash maps, as well as what makes each special.

## Index
1. Storing List of Values
2. Storing UTF-8 Encoded Text with Strings
3. Storing Keys with Associated Values in Hash Map

## 1. Storing List of Values with Vectors
The first collection type we will look at is `Vec<T>`, also known as a _vector_. Vectors allow you to store more than one value in a single data structure that puts all the values next to eact other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of the text in a file or the prices of items in a shopping cart.

### Creating a New Vectors
To create a new, empty vector, we can call the `Vec::new` function, as shown below

```rust
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here. Because we are not iserting any values into this vector, Rust does not know what kind of elements we intend to store. This is an important point. Vectors are implemented using _generics_. For now, know that the `Vec<T>` type provided by the standard library can hold any type, and when a specific vector holds a specific type, the type is specified within angle brackets. In the last code we told Rust thath `Vec<T>` in `v` will hold elements of the `i32` type.

In more realistic code, Rust can often infer the type value of youwant to store once you insert values, so rarely need to do this type annotation. It is more common to create a `Vec<T>` that has initial values, and Rust provides the `vec!` macro for convenienve. The macro will create a new `Vec<i32>` thath holds the values 1, 2 and 3.

```rust
let v = vec![1, 2, 3];
```

Because we have given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and the type annotation is not necessary. Next, we will look at how to modify a vector.

### Updating a Vector
To create a vector and then add elements to it, we can use the `push` method, as shown the next code:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
```

As with any variable, if we want to be able to change its values, we need to make it mutable using the `mut` keyword. The numbers we place inside are all of type `i32`, and Rust infers this from the data, so we do not need the `Vec<i32>` annotation.

### Dropping a Vector
Like any other `struc`, a vector is freed when it goes out of scope, as annotated the next snippet.

```rust
{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // v goes out of scope and is freed here
```

When the vector gets dropped, all of its contents are also dropped, meaning those integers it holds will be cleaned up. This may seem like a straightforward point but can get a bit more complicated when you start to introduce references to the elements of the vector. Let's tackle that next.


### Reading Elements of Vectors
Now that you know got to create, update, and destroy vectors, knowing how to read their contents is a good next step. There are two ways to reference a value stored in a vector. In the examples we have annotated the types of the values that are returned from these functions for extra clarity.

The snippet below shows both methods of accessing a value in a vector, either with indexing syntax or the `get` method.

```rust
let v = vec![1, 2, 3, 4];
let third: &i32 = &v[2];

println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element.")
}
```
Note two details here. First, we use the index value of 2 to get the third element. Vectors are indexed by number, starting at zero. Second, the two ways to get the third element are by useing `&` and `[]`, which gives us a reference, or by using the `get` method witht the index passed as an argument, which gives us an `Option<&T>`.

Rust has to ways to reference an element so you can choose how the program behaves when you try to use an index value that the vector does not have an element for. As an example, let's see what a program will do if it has a vector that holds five elements and then tries to access ans element at index 100, as shown next.

```rust
let v = vec![1, 2, 3, 4];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

When we run this code, the first `[]` method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there is an attempt to access an element past the end of the vector.

When the `get` method is passed an index that is outside the vector, it returns `None` without panicking. You would use this method if accessing an element beyond the range of the vector happens occasionally under normal circumstances. Your code will then have logic to handle having either `Some(&element)` or `None`. For ecample, the index could be coming from a person entering a number. If they accidentallly enter a nmber that is too large the program gets a `None` value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value. That would be more user-friendly than crashing the program due to a typo.

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid. Recall the rules that states you cannot have mutable and immutable references in the same scope. That rule applies in the next snippet, where we hold an immutable reference to the first element in a vector and try to add an element to the end, which won't work.

```rust
let mut v = vec![1, 2, 3, 4];
let first = &v[0];

v.push(6)
println!("The first element is: {}", first);
```

Compiling this code will result in this error:

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```

The last code might look like should work. Why should a reference to the first element care about what changes at the end of the vector? This error is due to the way vectors work. Adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there is not enough room to put all the elements next to each other where the vector currentlu is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs form ending up in that situation.

### Iterating over the Values in a Vectors
If we want to access each element in a vector in turn, we can iterate through all of the element rather than use indices to access one at time. Code below, shows how to use a `for` loop to get immutable references to each element in a vector `i32` values and print them.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

We can also iterate over mutable references to each in a mutable vector in order to make changes to all the elements. The `for` loop in the next code will add 50 to each element.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

To change the value that the mutable reference refers to, we have to use the derence operator ('*') to get to the value in `i` before we can use the `+=` operator. We will talk more about the dereference operator in the _following the pointer to the value with the dereference operator_ section of the chapter 15.

### Using an Enum to Store Multiple Types
At the beginning of this chapter, we said that vectors can only store values that are the same type. This can be inconvenient. There are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum.

For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type, enum. Then we can create a vector that holds that enum and so, ultimately, holds different types. We have demonstrated this below.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec! [
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Int(10.12),
]
```
Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. A secondory advantage is that we can be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled, as discussed in chapter 6.

When you are writing a program, if you do not know the exhaustive set of types the program will get at runtime to store in a vector, the enum technique won't work. Instead, you can use a trait object, which we will cover in chapter 17.

Now that we have discussed some of the most common ways to use vectors, be sure to review API documentation for all the many useful methods defined on `Vec<T>` by the standard library. For example, in addition to `push`, a `pop` method removes and return the last element. Let's move on the next collection type: `String`.

## 2. Storing UTF-8 Encoded Text with Strings

## 3. Storing Keys with Associated Values in Hash Map
