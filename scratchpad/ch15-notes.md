# Smart Pointers

A _pointer_ is a general concept for a variable that contains and address in memory. This address refers to or —points at— some other data. The most common kind of pointer in Rust is a reference, which you learned about in chapter 4. References are indicated by the `&` symbol and borrow the value they point to. They do not have any special capabilities other than referring to data. Also, they do not have any overhead and are the kind of pointer we use most often.

*Smart pointers*, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities. The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well. In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references. One example that we’ll explore in this chapter is the *reference counting* smart pointer type. This pointer enables you to have multiple owners of data by keeping track of the number of owners and, when no owners remain, cleaning up the data.

In Rust, which uses the concept of ownership and borrowing, an additional difference between references and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers *own* the data they point to.

We’ve already encountered a few smart pointers in this book, such as `String` and `Vec<T>` in Chapter 8, although we didn’t call them smart pointers at the time. Both these types count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with `String` ensuring its data will always be valid UTF-8).

Smart pointers are usually implemented using **structs**. The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers. The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope. In this chapter, we’ll discuss both traits and demonstrate why they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won’t cover every existing smart pointer. Many libraries have their own smart pointers, and you can even write your own. We’ll cover the most common smart pointers in the standard library:

- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

In addition, we’ll cover the *interior mutability* pattern where an immutable type exposes an API for mutating an interior value. We’ll also discuss `reference cycles`: how they can leak memory and how to prevent them.

Let’s dive in!

## Index
1. Using Box to Point to Data on Heap
2. Treating Smart Pointers Like Regular References with the Deref Trait
3. Running Code on Cleanup with the Drop Trait
4. Rc, the Reference Counted Smart Pointer
5. RefCell and the Interior Mutability Pattern
6. Reference Cycles Can Leak Memory

## 1. Using Box to Point to Data on Heap
The most straightforward smart pointer is a *box*, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. Refer to Chapter 4 to review the difference between the stack and the heap.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

We’ll demonstrate the first situation in the “Enabling Recursive Types with Boxes” section. In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap. The third case is known as a trait object, and Chapter 17 devotes an entire section, “Using Trait Objects That Allow for Values of Different Types,” just to that topic. So what you learn here you’ll apply again in Chapter 17!

### Using a Box to Store Data on the Heap
Before we discuss this use case for `Box<T>`, we’ll cover the syntax and how to interact with values stored within a `Box<T>`.

```rs
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

We define the variable `b` to have the value of a `Box` that points to the value `5`, which is allocated on the heap. This program will print `b = 5`; in this case, we can access the data in the box similar to how we would if this data were on the stack. Just like any owned value, when a box goes out of scope, as `b` does at the end of `main`, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).

Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in this way very often. Having values like a single `i32` on the stack, where they’re stored by default, is more appropriate in the majority of situations. Let’s look at a case where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.

### Enabling Recursive Types with Boxes
At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a *recursive type*, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

Let’s explore the *cons list*, which is a data type common in functional programming languages, as an example of a recursive type. The cons list type we’ll define is straightforward except for the recursion; therefore, the concepts in the example we’ll work with will be useful any time you get into more complex situations involving recursive types.

#### More Information About the Cons List
A `cons list `is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the cons function (short for “construct function”) constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.

The cons function concept has made its way into more general functional programming jargon: “to cons *x* onto *y*” informally means to construct a new container instance by putting the element *x* at the start of this new container, followed by the container *y*.

Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called `Nil` without a next item. A cons list is produced by recursively calling the cons function. The canonical name to denote the base case of the recursion is `Nil`. Note that this is not the same as the “null” or “nil” concept in Chapter 6, which is an invalid or absent value.

Although functional programming languages use cons lists frequently, the cons list isn’t a commonly used data structure in Rust. Most of the time when you have a list of items in Rust, `Vec<T>` is a better choice to use. Other, more complex recursive data types are useful in various situations, but by starting with the cons list, we can explore how boxes let us define a recursive data type without much distraction.

Next snippet contains an enum definition for a cons list. Note that this code won’t compile yet because the `List` type doesn’t have a known size, which we’ll demonstrate.

```rs
enum List {
    Cons(i32, List),
    Nil,
}
```

> Note: We are implementing a cons list that holds only `i32` values for the purposes of this example. We could have implemented it using generics, as we discussed in Chapter 10, to defin a cons list type that could store values of any type.

Using the `List` type to store the list `1, 2, 3` would look like 

```rs
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

The first Cons value holds 1 and another List value. This List value is another Cons value that holds 2 and another List value. This List value is one more Cons value that holds 3 and a List value, which is finally Nil, the non-recursive variant that signals the end of the list.


If we try to complete this code, we get the next error:

```
$ cargo run

Compiling cons-list v0.1.0 (file:///projects/cons-list) error [E0072]: recursive type `List` has infinite size

```

The error shows this type “has infinite size.” The reason is that we’ve defined List with a variant that is recursive: it holds another value of itself directly. As a result, Rust can’t figure out how much space it needs to store a `List` value. Let’s break down why we get this error a bit. First, let’s look at how Rust decides how much space it needs to store a value of a non-recursive type.

#### Computing the Size of a Non-Recursive Type
Recall the `Message` enum we defined in chapter 6 during the enum definitions:

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a `Message` value, Rust goes through each of the variants to see which variant needs the most space. Rust sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough space to store two `i32` values, and so forth. Because only one variant will be used, the most space a `Message` value will need is the space it would take to store the largest of its variants.

Contrast this with what happens when Rust tries to determine how much space a recursive type like the `List` enum needs. The compiler starts by looking at the `Cons` variant, which holds a value of type `i32` and a value of type `List`. Therefore, `Cons` needs an amount of space equal to the size of an `i32` plus the size of a `List`. To figure out how much memory the `List` type needs, the compiler looks at the variants, starting with the `Cons` variant. The `Cons` variant holds a value of type `i32` and a value of type `List`, and this process continues infinitely, as shown in Figure below:


![image](../assets/15-01_cons_list.svg)

#### Using Box to Get a Recursive Type with a Known Size
Rust can’t figure out how much space to allocate for recursively defined types, so the compiler gives the error. But the error does include this helpful suggestion:

```
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable
```

In this suggestion, "indirection" means that instead of storing a value directly, we will change the data structure to store the value indirectly by storing a pointer to the value instead.

Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead of another `List` value directly. The `Box<T>` will point to the next `List` value that will be on the heap rather than inside the `Cons` variant. Conceptually, we still have a list, created with lists “holding” other lists, but this implementation is now more like placing the items next to one another rather than inside one another.

We can change the definition of `List` enum and the usage of `List` code. The next code have these modifications and it will compile:

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

The `Cons` variant will need the size of an `i32` plus the space to store the box’s pointer data. The `Nil` variant stores no values, so it needs less space than the `Cons` variant. We now know that any `List` value will take up the size of an `i32` plus the size of a box’s pointer data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a `List` value. Next figure shows what the `Cons` variant looks like now.

![image](../assets/15-02_cons_list_hold_in_box.svg)

Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types. They also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need. We’ll look at more use cases for boxes in Chapter 17, too.

The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation. Let’s explore these two traits in more detail. These two traits will be even more important to the functionality provided by the other smart pointer types we’ll discuss in the rest of this chapter.

## 2. Treating Smart Pointers Like Regular References with the Deref Trait

Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator*, `*` (as opposed to the multiplication or glob operator). By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

Let’s first look at how the dereference operator works with regular references. Then we’ll try to define a custom type that behaves like `Box<T>,` and see why the dereference operator doesn’t work like a reference on our newly defined type. We’ll explore how implementing the Deref trait makes it possible for smart pointers to work in ways similar to references. Then we’ll look at Rust’s *deref coercion* feature and how it lets us work with either references or smart pointers.

> Note: There is one big difference betweent the `MyBox<T>` type we are about to built and the real `Box<T>`: our version will not store its data on the heap. We are focusing this example on `Deref`, so where the data is actually stored is less important than the pointer-like behavior.

### Following the Pointer to the Value with the Dereference Operator
A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else. In the next snippet, we create a reference to an i32 value and then use the dereference operator to follow the reference to the data:

```rs
fn main() {
   let x = 5;
   let y = &x;
   
   assert_eq!(5, x);
   assert_eq!(5, *y);
}
```

The variable `x` holds an `i32` value, `5`. We set `y` equal to a reference to `x`. We can assert that `x` is equal to `5`. However, if we want to make an assertion about the value in `y`, we have to use `*y` to follow the reference to the value it’s pointing to (hence *dereference*). Once we dereference `y`, we have access to the integer value `y` is pointing to that we can compare with `5`.

If we tried to write `assert_eq(5, y);` instead we get a compilation error. Comparing a number and a reference to a number isn’t allowed because they’re different types. We must use the dereference operator to follow the reference to the value it’s pointing to.

### Using Box Like a Reference
We can rewrite the last snippet using `Box<T>` instead of a reference. The dereference operator will work as shown below:

```rs
fn main() {
   let x = 5;
   let y = Box::new(x);
   
   assert_eq!(5, x);
   assert_eq!(5, *y);
}
```

The only difference between the last two code sample is that here we set `y` to be an instance of a box pointing to the value in `x` rather than a reference pointing to the value of `x`. In the last assertion, we can use the dereference operator to follow the box’s pointer in the same way that we did when `y` was a reference. Next, we’ll explore what is special about `Box<T>` that enables us to use the dereference operator by defining our own box type.

### Defining Our Own Smart Pointer
Let’s build a smart pointer similar to the `Box<T>` type provided by the standard library to experience how smart pointers behave differently from references by default. Then we’ll look at how to add the ability to use the dereference operator.

The `Box<T>` type is ultimately defined as a tuple struct with one element, so the next implementation defines a `MyBox<`T> type in the same way. We’ll also define a new function to match the new function defined on `Box<T>`.

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

We define a struct named `MyBox` and declare a generic parameter `T`, because we want our type to hold values of any type. The `MyBox` type is a tuple struct with one element of type `T`. The `MyBox::new` function takes one parameter of type T and returns a `MyBox` instance that holds the value passed in.

Let’s try adding the `main` function in the last codes and changing it to use the `MyBox<T>` type we’ve defined instead of `Box<T>`. The next code won’t compile because Rust doesn’t know how to dereference `MyBox`.


```rs
fn main() {
   let x = 5;
   let y = MyBox::new(x);
   
   assert_eq!(5, x);
   assert_eq!(5, *y);
}
```

Here is the resulting compilation error:

```rs
$ cargo run
    Compiling deref-example v0.1.0 (file:///projects/deref-example)
    error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
```

Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that ability on our type. To enable dereferencing with the `*` operator, we implement the `Deref` trait.

### Treating a Type Like a Reference by Implementing the Deref Trait
As discussed in Chapter 10, to implement a trait, we need to provide implementations for the trait’s required methods. The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows self and returns a reference to the inner data. Next snippet contains an implementation of `Deref` to add to the definition of `MyBox`:

```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}
```

The type `Target = T;` syntax defines an associated type for the `Deref` trait to use. Associated types are a slightly different way of declaring a generic parameter, but you don’t need to worry about them for now; we’ll cover them in more detail in Chapter 19.

We fill in the body of the `deref` method with `&self.0` so `deref` returns a reference to the value we want to access with the `*` operator. The `main` function that calls `*` on the `MyBox<T>` value now compiles, and the assertions pass!

Without the `Deref` trait, the compiler can only dereference `&` references. The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference.

When we entered `*y` in main function that use the MyBox, behind the scenes Rust actually ran this code:

```
*(y.deref())
```

Rust substitutes the `*` operator with a call to the `deref` method and then a plain dereference so we don’t have to think about whether or not we need to call the `deref` method. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements `Deref`.

The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary, is the ownership system. If the `deref` method returned the value directly instead of a reference to the value, the value would be moved out of self. We don’t want to take ownership of the inner value inside `MyBox<T>` in this case or in most cases where we use the dereference operator.

Note that the `*` operator is replaced with a call to the `deref` method and then a call to the `*` operator just once, each time we use a `*` in our code. Because the substitution of the `*` operator does not recurse infinitely, we end up with data of type `i32`, which matches the `5` in `assert_eq!` in the main function that use our `MyBox` implementation

### Implicit Deref Coercions with Function and Methods
*Deref coercion* is a convenience that Rust performs on arguments to functions and methods. Deref coercion works only on types that implement the `Deref` trait. Deref coercion converts such a type into a reference to another type. For example, deref coercion can convert `&String` to `&str` because String implements the `Deref` trait such that it returns `str`. Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with `&` and `*`. The deref coercion feature also lets us write more code that can work for either references or smart pointers.

To see deref coercion in action, let's use the `MyBoxM<T>` as well as the implementation of `Deref`. Next snippet shows the definition of a function that has a string slice parameter:

```rs
fn hello (name: $str) {
    println!("Hello, {}!", name);
}
```
We can call the `hello` function with a string slice as an argument, such as `hello("Rust");` for example. Deref coercion makes it possible to call `hello` with a reference to a value of type `MyBox<String>`, as shown next:

```rs
fn main() {
   let m = myBox::new(String::from("Rust"));
   hello(&m);
}
```

Here we’re calling the `hello` function with the argument `&m`, which is a reference to a `MyBox<String>` value. Because we implemented the `Deref` trait on `MyBox<T>`, Rust can turn `&MyBox<String>` into `&String` by calling deref. The standard library provides an implementation of `Deref` on `String` that returns a string slice, and this is in the API documentation for `Deref`. Rust calls deref again to turn the `&String` into `&str`, which matches the hello function’s definition.

If Rust didn’t implement deref coercion, we would have to write the code to call `hello` with a value of type `&MyBox<String>`.


```rs
fn main() {
   let m = myBox::new(String::from("Rust"));
   hello(&(*m)[..]);
}
```

The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and `[..]` take a string slice of the `String` that is equal to the whole string to match the signature of `hello`. The code without deref coercions is harder to read, write, and understand with all of these symbols involved. Deref coercion allows Rust to handle these conversions for us automatically.

When the `Deref` trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter’s type. The number of times that `Deref::deref` needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

### How Deref Coercion Interacts with Mutability

Similar to how you use the `Deref` trait to override the `*` operator on immutable references, you can use the `DerefMut` trait to override the `*` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same except for mutability. The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can get a `&U` transparently. The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that. Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.

## 3. Running Code on Cleanup with the Drop Trait
## 4. Rc, the Reference Counted Smart Pointer
## 5. RefCell and the Interior Mutability Pattern
## 6. Reference Cycles Can Leak Memory
