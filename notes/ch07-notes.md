# Chapter 07: Managing Growing Projects with Packages, Crates, and Modules.

As you write large programs, organizing your code will be important because keeping track of your entire program in your head will become impossible. By grouping related functionality and separating code with distinct features, you will clarify where to find code that implements a particular feature and where to go to change how a feature works.

The programs we have written so far have been one module in one file. As a project grows, you can organize code by splitting it into multiple modules and then multiple file. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. This chapter covers all these techniques. For very large projects of a set of interrelated package that evolve together, Cargo provides workspaces, which we will cover in chapter 14.

In addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level. Once you have implemented an operation, other code can call that code via the code's public interface without knowing how the implementation works. The way you write code define which parts are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head.

A related concept is **scope**, the nested context in which code is written has a set of names that are defines as _in scope_. When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spots refers to a variable, function, struct, enum, module, constant, or other item and what item means. You can create scopes and change which names are in or out of scope. You cannot have two items with the same name in the same scope, tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code's organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the _module system_ and include.

+ **Packages**: A Cargo feature that lets you build, test, and share crates.
+ **Crates**: A tree of modules that produces a library or executable.
+ **Modules and use**: Let your control the organization, scope, and privacy paths.
+ **Paths**: A way of naming an item, such as a struct, function, or module

In this chapter, we will cover all these features, discuss how they interact, and explain how to use them to manage scope. By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro.

## Index
1. Packages and Crates
2. Defining Modules to Control Scope and Privacy
3. Paths to Referring to an Item in the Module Tree
4. Bring Paths Into Scope with the `use` Keyword
5. Separating Modules into Different Files.

## 1. Packages and Crates
The first parts of the module system we will cover are package and crates. A crate is a binary or a library. The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate. A _package_ is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

Several rules determine what a package can contain. A package must contain zero or one library crates, and no more. It can contain as many binary crates as you would like, but it must contain at least one crate.

Let's walk through what happens when we create a package. First, we enter the command `cargo new`:

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

When we entered the command, Cargo created a `Cargo.toml` file, giving us a package. Looking at the contents of `Cargo.toml` there is no mention of `src/main.rs` because Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root. cargo passes the crate root files to `rustc` to build the library or binary.

Here we have a package that only contains `src/main.rs` meaning it only contains a binary crate named `my-project`. If a package contains `src/main.rs` and `src/lib.rs`, it has two crates. A library and a binary, both with the same name as the package. A package can have multiple binary crates placing in the `src/bin` directory, each file will be a separate binary crate.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects. For example, the `rand` crate used in the guessing number game, provides functionality that generates random numbers. We can use that functionality in our projects by bringing the `rand crate into our project's scope. All the functionality provided by the `rand` crate is accessible through a crate's name, `rand`.

Keeping a crate's functionality in its own scope clarifies whether particular functionality is defined in our crate or the `rand` crate and prevents potential conflicts. For example, the `rand` crate provides a trait name `Rng`. we can also define `struct` named `Rng` in our crate. Because a crate's functionality is namespaced in its own scope, when we add `rand` as a dependency, the compiler is not confused about what the name `Rng` refers to. In our crate, it refers to the `struct Rng` that we defined. We would access the `Rng` trait from the `rand` crate as `rand::Rng`.

## 2. Defining Modules to Control Scope and Privacy
_Modules_ let us organize code within a crate into groups for readability and easy reuse. Modules also control the _privacy_ items, which is whether an item can be used by outside code (public) or is internal implementation detail and not available for outside use (private).

As an example, let's write a library crate that provides the functionality of a restaurant. We will define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than actually implement a restaurant code.

In the restaurant industry, some parts of a restaurant are referred to as _front of house_ and others a_back of house_. Front of house is where customers are, this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of the house is where the chefs and cooks work in the kitchen, dishwasher clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize the function into nested modules. Create a new library named `restaurant` by running `cargo new --lib restaurant`, then put the code in `src/lib.rs` to define some modules and function signatures.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

We define a module by starting with the `mod` keyword and then specify the name of the module (`front_of_house`) and place curly bracket around the body of the module. Inside modules, we can have other modules, as in this case with `hosting` and `seving`. Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions.

By using modules, we can group related definitions together and name why they are related. Programmers, using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather that having to read through all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that `src/main.rs` and `src/lib.rs` are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at root of the crate's module structure, known as the **module tree**.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

This tree shows how some of the modules nest inside one another. The three also shows that some modules are _siblings_ to each other, meaning they are defined in the same module. To continue with the family metaphor, if module A is contained inside module B, we say that module A is the child of module B and the module B is the _parent_ of module A. Notice that the entire module tree is rooted under the implicit module named `crate`.

This module tree might remind you of the file system's directory tree on your computer. This is a very apt comparison. Just like directories in a file system, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

## 3. Paths to Referring to an Item in the Module Tree

## 4. Bring Paths Into Scope with the `use` Keyword

## 5. Separating Modules into Different Files.

