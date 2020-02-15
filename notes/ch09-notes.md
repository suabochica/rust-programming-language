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
## 3. To panic! or Not to panic!
