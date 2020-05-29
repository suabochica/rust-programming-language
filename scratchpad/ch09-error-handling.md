# Chapter 09: Error Handling
Rust's commitment to reliability extends to error handling. Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledfe the possibility of an error and take some action before your code will compile. This requirement makes your program more robust by ensuring that you will discover errors and handle them appropriately before you have deployed your code to production.

Rust groups errors into two major categories:

1. Recoverable
2. Unrecoverable

For recoverable error, such as a file not found error, it is reasonable to report the problem to the user and retry the operation.

For unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages do not distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust does not have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters and unrecoverable error. This chapter covers calling `panic!` first and then talks about returning `Result<T, E>` values. Additionally, we will explore considerations when deciding whether to try to recover from an error or to stop execution.

## Index
1. Unrecoverable errors with panic!
2. Recoverable errors with Result
3. To panic! or Not to panic!

## 1. Unrecoverable errors with panic!
Sometimes, bad things happen in your code, and there is nothing you can do about it. In these cases, Rust has the `panic!` macro. When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. This most commonly occurs when a bug of some kind has been detected and it is not clear to the programmer how to handle the error.

Let's try calling `panic!` in a simple program:

```rust
fn main() {
    panic!("crash and burn");
}
```

When you run the program, you will see something like this:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The call to `panic!` causes the error message contained in the last two lines. The first line shows our panic message and the place in our source code where the panic ocurred.

In this case, the line indicated is part of our code, and if we go to that line, we see the `panic!` macro call. In other cases, the `panic!` call might be in code that our code calls, and the filename and line number reported by the error message will be someone else's code where the `panic!` macro is called, not the line of our code that eventually led to the `panic!` call. We can use the backtrace of the functions the `panic!` call came from to figure out the part of our code that is causing the problem. We will discuss a backtrace is in more detail next.

### Using a `panic!` Backtrace.
Let's look another example to see what it is like when a `panic!` call comes from a library because of a bug in our code instead of from our code calling the macro directly.

```rust
fn main() {
    let v = vec![1, 2, 3];
    
    v[99];
}
```

Here we are attempting to access the 100th element of our vector, but it has only 3 elements. In this situation, Rust will panic. Using `[]` is suppossed to return an element, but if you pass an invalid index, there is no element that Rust could return here that would be correct.

Other languages, like C, will attempt to give you exactly what you asked for in this situation, even though it is not, what you want. You will get whatever is at the location in memory that would correspond to that element in the vector, even though the memory does not belong to the vector. This is called a _buffer overread_ and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they should not be allowed to that is stored after the array.

To protect your program from this sort of vulnerability, if you try to read an element at an index that does not exist. Rust will stop execution and refuse to continue. Let's try it an see:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This error points at a file we did not write, `libcore/slice/mod.rs`. That is the implementation of `slice` in the Rust source code. The code that gets run when we use `[]` on our vector `v` is in `libcore/slice/mod.rs`, and that is where `panic!` is actually happening.

The next note line tells us that we can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error. A _backtrace_ is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages. The key to reading the backtrace is to start from the top and read until you see files you wrote. That is the spot where the problem originated. The lines above the lines mentioning your files are code that your code called. The lines below are code that called your code. These lines might include core Rust code, standard library code, or creates that you are using. Let's try getting a backtrace by setting the `RUST_BACKTRACE` environment variable to any value except 0.

```
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
   ...
```

That is a lot of output! The exact output you see might be different depending on your operating system and Rust version. In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enable by default when using `cargo build` or `cargo run` without the `--release` flag. as we have here.

In the last output, in some place you will identify the line of the backtrace points to the line in our project that is causing the problem, line `src/main.rs`. If we do not want our program to panic, the location pointed to by the first line mentioning a file we wrote is where we should start investigating. In the output wihtout `RUST_BACKTRACE`, where we deliberately wrote code that would panic in order to demonstrate how to use backtraces, the way to fic the panic is to not request an element at index 99 from a vector that only contains 3 items. When your code panics in the future, you will need to figure out what action the code is taking with what values to cause the panic and what the coude should do instead.

We will come back to `panic!` and when we should and should not use `panic!` to handle error conditions in the third section. Next, we will look at how to recover from an error using `Result`.

## 2. Recoverable errors with Result
Most errors are not serious enough to require the program to stop entirely. Sometimes, when a function fails, it is for a reasion that you can easily interpret and respond to. For example, if you try to open a diel and that operation fails because the file does not exist, you might want to create the file instead of terminating the process.

Recall from _handling potential failure with `Result` type_ in chapter 2 that the `Result` enum is defined as having two variants, `Ok` and `Err` as follows.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters. We will discuss generics in more detail in chapter 10. What you need to know rigth now is that `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant. Because `Result` has these generic type parameters, we can use the `Result` type and the functions that the standard library has defined on it in many different situations where the sucessful value and error value we want to return may differ.

Let's call a function that returns a `Result` value because the function could fail. In the nexto code we try to open a file.

```rust
use std:fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

How do we know `File::open` returns a `Result`? We could look at the standard library API documentation, or we could ask the compiler. If we give `f` a type annotation that we know is _not_ the return type of the function and then try to compile the code, the compiler will tell us that the types do no match. The error message will then tell us what the type of `f` is. Let's try it. We know that the return type of `File::open` is not of type `u32`, so let's change the `let f` statement to:

```rust
let f: u32 = File::open("hello.txt");
```

Attempting to compile now give us the following output:

```
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
             found type `std::result::Result<std::fs::File, std::io::Error>`
```

This tell us the return type of the `File::open` function is a `Result<T, E>`. The generic parameter `T` has been filled in here with the type of the success value, `std::fs::File`, which is a file handle. The type `E` used in the error value is `std::io:Error`.

This return type means the call to `File::open` migth succeed and return a file handle that we can read from or write to. The `File::open` function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. This information is exactly what the `Result` emum conveys.

In the case where `File::open` succeds, the value in the variable `f` will be an instance of `Ok` that contains a file handle. In the case where it fails, the value in `f` will be an instance of `Err` that contains more information about the kind of error that happpened.

We need to add in the last code a logic to take different action depending on the value `File::open` retunrs. Below, we shows one way to handle the `Result` using a basic tool, the `match` expression that we discussed in chapter 6.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file {:?}", error)
        },
    };
}
```

Note that, like the `Option` enum, the `Result` enum and its variants have been brought into scope by the prelude, so we do not need to specify `Result::` before the `Ok` and `Err` variants in the `match` arms.

Here we tell Rust that when the result is `Ok`, return the inner `file` value out of the `Ok` variant, and we then assign that file handle for reading or writing.

The other arm of the `match` handles the case where we get an `Err` value from `File::open`. In this example, we have chosen to call the `panic!` macro. If there is no file named `hello.txt` in our current directory and we run this code, we will see the following output from the `panic!` macro:

```
thread 'main' panicked at 'Problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```

As usual, this output, tells us exactly what has gone wrong.

### Matching on Different Errors
Theco last code will `panic!` no matter why `File::open` failed. What we want to do instead is take different actions for different failure reasons. If `File::open` failed because the does not exist, we want to create the fiel and retunr the handle to the new file. If `File::open` failed for any reason, like we did not have permission to open the file, we still want the code to `panic!` in the same way as we did before. Check the next code which add an inner `match` expression.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = file::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    }
}
```

The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library. This struct has a method `kind` that we can call to get an `io::ErrorKind` value.  The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of that might result from an `io` operation. The variant we want to use is `ErrorKind::NotFound`, which indicates the file we are trying to open does not exist yet. So we match on `f`, but we also have an inner match on `error.kind()`.

The condition we want to check in the inner match is whether the value returned by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is, we try to create the file with `File::create`. However, because `File::create` could also fail, we need a second arm in the inner `match` expression. When the file cannot be created a different error message is printed. The second arm of the outer `match` stays the same, so the program panics on any error besides the missing file error.

That is a log of `match`. The `match` expression is very useful but also very much a primitive. In chapter 13, you will learn about closures. The `Result<T,E>` type has many methods that accept a closure and are implemented using `match` expressions. Using those methods will make your code more concise. A more seasoned Rustacean might write this code instead.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = file::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}
```

Although this code has the same behavior in the nested match code, it does not contain any `match` expressions and is cleaner to read. Come back to this example after you have read chapter 13, and look up the `unwrap_or_else` method in the standard library documentation. Many more of these methods can clean up huge nested `match` expressions when you are dealing with errors.

### Shortcuts for Panic on Error
Using `match` works well enough, but it can be a bit verbose and does not always communicate intent well. The `Result<T,E>` type has many helper methods defined on it to do various tasks. One of those methods, called unwrap, is a shortcut method that is implemented just like the `match` expression we wrote before. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, unwrap will call the `panic!` macro for us. Here is an example of `unwrap` in action:

```rust
use std::fs::file;

fn main() {
    let f = file::open("hello.txt").unwrap();
}
```

If we run this code without a `hello.txt` file, we will see an error message from the `panic!` call that the `unwrap` method makes:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

Another method, `expect`, which is similar to `unwrap`, lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of `expect` looks like this:

```rust
use std::fs::file;

fn main() {
    let f = file::open("hello.txt").expect("Failed to open hello.txt");
}
```

We use `expect` in the same way as `unwrap`, to return the file handle or call the `panic!` macro. The error message used by `expect` in its call to `panic!` will be the parameter that we pass to `expect`, rather than the default `panic!` message that `unwrap` uses.

```
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Because this error message starts with the text we specified, `Failed to open hello.txt`, it will be easier to find where in the code this error message is coming from. If we use `unwrap` in multiple places, it can take more time to figure out exactly which `unwrap` is causing the panic because all `unwrap` call that panic print the same message.

### Propagating Errors
When you are writing a function whose implementation calls something that might fall, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as _propagating_ the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of you code.

For example, this snippet shows a function that reads a username from a file. If the file does not exist or cannot be read, this function will return those errors to the code that called this function.

```rust
use std::io;
use std::io::READ;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}
```

This function could be written in a shorter way, but we are going to start by doing a lot of it manually in order to explore error handling. At the end we will show the shorter way.

Let's look at the return type of the function first: `Result<String, io:Error>`. This means the function is returning a value of type `Result <T, E>` where the generic parameter `T` has been filled in with the concrete type `String` and the generic type `E` has been filled in with the concrete type `io::Error`. If this function succeeds without any problem, the code that calls this function will receive an `Ok` value that holds a String (the username that this function read from the file). If this function encounters any problems, the code that calls this function will receive and `Err` value  that holds ans instance of `io::Error` that contains more information about what the problems were. We chose `io::Error` as the return type of this function because that happens to be the type of the error value returned from both of the operations we are calling in this function's body that might fails. The `File::open` function and the `read_to_string` method.

The body of the function starts by calling the `File::open` function. Then we handle the `Result` value returned with a `match`, passing the error value from `File::open` back to the calling code as this function's error values. If `File::open` succeeds, we store the file handle in the variable `f` and continue

Then we create a new `String` in variable `s` and call the `read_to_string` method on the file handle in `f` to read the contents of the file into `s`. The `read_to_string` method also return a `Result` because it might fail, even though `File::open` succeeded. So, we need another `match` to handle the `Result`. If `read_to_string` succeeds, then our function has succeeded, and we return the username from the files that is now in `s` wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the same way that we returned the error value in the `match` that handled the return value of `File::open`. However, we do not need to explicitly say `return`, because this is the last expression in the function.

The code that calls this code will then handle getting either an `Ok` value that contains a username or an `Err`, value that contains an `io::Error`. We do not know what the calling code will do with those values. If the calling code gets and `Err` value, it could call `panic!` and crash the program, use a default username, or look up the username from somewhere other than a file, for example. We do not have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately.

This pattern of propagating error is so common in Rust that Rust provides the question mark operator `?` to make this easier.

#### The `?` Operator
Let's replace our last implementation of `read_username_file` using the `?` operator.

```rust
use std::io;
use std::io::Read;
use std::io::File;

fn read_username_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
The `?` placed after `Result` value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` values. If the value of the `Result` is an `Ok`, the value inside `Ok` will get returned from this expression, and the program will continue. If the values is `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error values gets propagated to the calling code.

There is a difference between what the `match` expression from the version without `?` and with `?`: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert error from one type into another. When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function return one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the `from` function to define how to convert itself to the returned error type, the `?` operator takes care of the conversion automatically.

In the context of the last snippet, the `?` at the end of the File::open call will return the value inside an `Ok`to the variable `f`. If an error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code. The same thing applies to the `?` at the end of the `read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function's implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?`, as shown below:

```rust
use std::io;
use std::io::Read;
use std::io::File;

fn read_username_file() -> Result<String, io::Error> {
    let mut s = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

We have moved the creation of the new `String` in `s` to the beginning of the function. That part has not changed. Instead of creating a variable `f` we have chained the call to `read_to_string` directly onto the result of `File::open("hello.txt")?`. We still have a `?` at the end of the `read_to_string` call, and we still return an `Ok` value containing the username in `s` when both `File::open` and `read_to_string` succeed rather than returning errors. The functionality is again the same as in old versions, with the difference that this version is more ergonomic way to write it.

Speaking of different ways to write this function, we have an even shorter version shown below:

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Reading a file into a string is fairly common operation, so Rust provides a convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns its. Of course, using `fs::read_to_string` does not give us the opportunity to explain all the error handling, so we did it the longer way first.

#### The `?` Operator on `Result` returns
The `?` operator can be used in functions that have a return type of `Result`, because it is defined to work in the same way as the `match` expressions we defined before. The part of the `match` that requires a return type of `Result`is `return Err(e)`, so the return type of the function can be `Result` to be compatible with this `return`.

Let's look at what happens if we use the `?` operator in the `main` function, which you will recall has a return type of `()`:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

When we compile this code, we get the following error message:

```
error[E0277]: the `?` operator can only be used in a function that returns
`Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
   |
   4 |     let f = File::open("hello.txt")?;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a
       function that returns `()`
         |
           = help: the trait `std::ops::Try` is not implemented for `()`
           = note: required by `std::ops::Try::from_error`
```
This error points out that we are only allowed to use the `?` operator in a function that returns `Result`or `Option`or another type that implements `std::ops::Try`. When you are writing code in a function that does not return one of these types, an you want to use `?` when you call other function that return `Result<T, E>`, you have two choices to fix this problem. One technique is to change the return type of your function to be `Result<T,E>` if you have no restrictions preventing that. The other technique is to use a `match` or one of the `Result<T, E>` method to handle the `Result<T, E>`in whatever way is appropriate.

The `main` function is special, and there are restrictions on what its return type must be. One valid return type for main is `()`, and conveniently, another valid return type is `Result<T, E>`, as shown here:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hellow.txt")?;
    
    Ok(())
}
```
The `Box<dyn Error>` type is called a trait object, which we will talk about in the _Using Trait Objects that Allow for Values of Different Types_ section in chapter 17. For now, you can read `Box<dyn Error>` to mean "any kind of error". Using `?` in the `main` function with this return type is allowed.

Now that we have discussed the details of calling `panic!` or returning `Result`, let's return to the topic of how to decide which is appropriate to use in which cases.

## 3. To panic! or Not to panic!
So how do you decide when you should call `panic!` and when you should return `Result`? When code panics, there is no way to recover. You could call `panic!` for any situation, whether there is a possible way to recover or not, but then you are making the decision behalf of the code calling your code that a situation is unrecoverable. When you choose to return a `Result` value, you give the calling code options rather than making the decision for it. The calling code could choose to attempt to recover in a way that is appropriate for its situation, or it could decide that an `Err` value in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one. Therefore, returning `Result` is a good default choice when you are defining a function that might fail.

In rare situation, it is more appropriate to write code that panics instead of returning a `Result`. Let's explore why it is appropriate to panic in examples, prototype code, an tests. Then we will discuss situations in which the compiler cannot tell that failure is impossible, but you as a human can. The chapter will conclude with some general guidelines on how to decide whether to panic in library code.

### Examples, Prototype Code, and Tests
When you are writing an example to illustrates some concept, having robust error-handling code in the example as well can make the example less clear. In examples, It is understood that a call to a method like `unwrap` that could panic is meant as a placeholder for the way you would want your application to handle errors, which can differ based on what the rest of your code is doing.

Similarly, the `unwrap` and `expect` methods are very handy when prototyping, before you are ready to decide how to handle errors. They leave clear markers in your code for when you are ready to make your program more robust.

If a method call fails in a test, you'd want the whole test to fail, even if that method is not the functionality under test. Because `panic!` is how test is marked as a failure, calling `unwrap` or `expect`is exactly what should happen.

### Cases in Which You Have More Information Than the Compiler
It would also appropriate to call `unwrap` when you have some other logic that ensure the `Result` will have an `Ok` value, but the logic is not something the compiler understands. You will still have a `Result` value that you need to handle. Whatever operation you are calling still has the possibility of failing in general, even though it is logically impossible in your particular situation. If you can ensure by manually inspection the code that you will never have an `Err` variant, it is perfectly acceptable to call `unwrap`. Here is an example:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

We are creating an `IpAddr` instance by parsing a hardcoded string. We can see that `127.0.0.1` is a valid IP address, so it is acceptable to use `unwrap`here. However, having a hardcoded, valid string does not change the return type of the `parse` method. We still get a `Result` value, and the compiler will still make us the `Result`as if the `Err` variant is a possibility because the compiler is not smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure, we would definitely want to handle the `Result`in a more robust way instead.

### Guidelines for Error Handling
It is advisable to have your code panic when it is possible that your code could end up in a bad state. In this context, a _bad state_ is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code, plus:

+ The bad state is not something that is _expected_ to happen occasionally.
+ Your code after this point needs to rely on not being in this bad state.
+ There is not a good way to encode this information in the types you use.

If someone calls your code and passes in values that do not make sense, the best choice might be to call `panic!`and alert the person using your library to the bug in their code so the can fix it during development. similarly, `panic!` is often appropriate if you are calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it is more appropriate to return a `Result` than to make a `panic!` call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide hot to handle.

When you code performs operations on values, your code should verify the values are valid first an panic if the values are not valid. This is most for safety reasons. Attempting to operate on invalid data can expose your code to vulnerabilities. This is the main reason the standard library will call `panic!` if you attempt an out-of-bounds memory access. Trying to access memory that does not belong to the current data structure is a common security problem. Functions often have _contracts_. Their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it is not a kind of error you want to calling code to have to explicitly handle. In fact, there is no reasonable way for calling code to recover. The calling _programmers_ need to fix code. Contract for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.

However, having lots of error checks in all of your functions would be verbose and annoying. Fortunately,
you can use Rust's type system to do many of the checks for you. If your function has a particular type as a parameter, you can proceed with your code's logic knowing that the compiler has already ensured you have a valid values. For example, if you have a type rather than an `Option`, your program expects to have _something_ rather than _nothing_. Your code then does not have to handle two cases for the `Some` and `None` variants: it will only have one case for definitely having a value. Code trying to pass nothing to your function won't even compile, so your function does not have to check for that case at runtime. Another example is using an unsigned integer type such as `u32`, which ensure the parameter is never negative.

### Creating Custom Type for Validation

Let's take the idea of Rust's type system to ensure we have a valid value one step further and look at crating a custom type for validation. Recall the guessing game in which our code asked the user to guess a number between 1 and 100. We never validated that the user's guess was between those numbers before checking it against our secret number. We only validated that the guess was positive. In this case, the consequences were not very dire. Our output of "Too high" or "Too low" would still be correct. But it would be a useful enhancement to guide the user toward valid guesses and have different behavior when a user guesses a number that is out of range versus when a user types, for example, letters instead.

One way to do this would be to parse the guess as an `i32` instead of only a `u32` to allow ptentially negative numbers, and then add a check for the number being in range, like so:

```rust
loop {
    // -- snip --
    let guess: i32 = match guess.trim().parse {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    if guess < 1 || guess> {
        prinln!("The secret number will be between 1 and 100");
        continue;
    }

    match guess.cmp(&secret_number) {...}
}
```

The `if` expression check whether our value is out of range, tells the user about the problem, and calls `continue`to start the next iteration of the loop and ask for another guess. After the `if` expression, we can proceed with the comparison between `guess` and the secret number knowing that `guess` is between 1 and 100.

However, this is not an ideal solution. If it was absolutely critical that program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious.

Instead, we can make a new type and put the validations in a function to create an instance of the type rather than repeating the validation everywhere. That way, it is safe for functions to use the new type in their signatures and confidently use the values they receive. Below we show a way to define a `Guess` type that will only create an instance of `Guess` if the `new` function receives a values between 1 and 100.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value)
        }
        
        Guess {
            value
        }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}
```
First, we define a struct named `Guess` that has a field named `value` that holds an `i32`. This is where the number will be stored.

Then we implement an associated function named `new` on `Guess` that creates instances of `Guess` values. The `new` function tests `values` to makes sure it is between 1 and 100. If `value` does not pass this test, we make a `panic!` call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a `Guess` with a `value` outside this range would violate the contract that `Guess::new` is relying on. The conditions in which `Guess::new` might panic should be discussed in it public-facing API documentation. We will cover documentation conventions indicating the possibility of a `panic!` in the respective section of chapter 14. If `value` does pass the test, we create a new `Guess` with its `value` field set to the `value` parameter and return the `Guess`.

Next, we implement a method named `value` that borrows `self`, does not have any other parameters, and return an `i32`. This kind of method is sometimes called a _getter_, because its purpose is to get some data from its fields and return it. This public method is necessary because the `value` field of the `Guess` struct is private. It is important that the `value` field be private so code using the `Guess` struct is not allowed to set `value` directly. Code outside the module _must_ use the `Guess::new` function to create an instance of `Guess`, thereby ensuring there is no way for a `Guess`to have a `value` that has not been checked by the conditions in the `Guess::new` function.

A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or return a `Guess` rather than an `i32`and would not need to do any additional checks in its body.

## Summary
Rust's error handling features are designed to help you write more robust code. The `panic!` macro signals that your program is in a state it cannot handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The `Result` enum uses Rust's type system to indicate that operations might fail in a way that your code could recover from. You can use `Result` to tell code that calls your code that it needs to handle potential success or failure ass wee. Using `panic!` and `Result` in the appropriate situations will make your code more reliable in the face of inevitable problems.

Now that you have seen useful ways that the standard library uses generics with the `Option` and `Result` enums, we will talk about how generics work an how you can use them in your code.
