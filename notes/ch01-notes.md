# Chapter 1: Getting Started
Let's start Rust Journey!. There is a lot to learn but every journey starts somewhere. In this chapter we will discuss.

+ Installing Rust on Linux, macOS and Windows.
+ Writing a program that prints `Hello, world!`
+ Using `cargo`, Rusts's packages manager and build.

## Index
1. Installation
2. Hello, World!
3. Hello, Cargo!

## 1.1. Installation
The first step is to install Rust. We will download Rust through `rustup`, a command line tool for managing Rust versions and associated tools. You will need and Internet connection for the download.

> Note: If you prefer not to use `rustup` for some reason, please see the [the Rust installation page](https://www.rust-lang.org/tools/install) for the options.

### Installing `rustup` on Linux or macOS
If you are using Linux or macOS, open a terminal and enter the following command:

```
$ curl https://sh.rustup.rs -sSf | sh
```

The command downloads a scripts and start the installation of the `rustup` tool. which install the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```
Rust is installed now. Great!
```

Feel free to download the script and inspect it before running it.

The installation script adds Rust to your system `PATH` after you next login. If you want to start using Rust right away instead of restarting your terminal, run the following command in your shell to add Rust to your system `PATH` manually.

```
$ source $HOME/.cargo/env
```

Alternatively, you can add the following line to your `~/.bash_profile`:

```
$ export PATH="$HOME/.cargo/bin:$PATH"
```

### Updating and Uninstalling
After you have installed rust Rust via `rustup`, updating to the latest version is easy. From your shell, run the following update script:

```
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your shell.

```
$ rustup self uninstall
```

## 1.2. Hello, World!
Now you have installed Rust, let;s writhe you first Rust program. It is traditional when learning a new language to write a little program that prints the text `Hello, world!` to the screen, so we will do the same here.

### Creating a Project Directory
You will start by making a directory to store your Rust code. It does not matter to Rust where your code lives. but for exercises and project in this book we suggest making a `/projects`directory in your home directory and keeping all your projects there.

Open a terminal and enter the following commands to make `/projects` directory and a directory for your "Hello, world!" program

> Note: In this repository we will use the `rust-programming-language` directory to host our Rust projects.

### Writing and Running a Rust program
Next, make a new source file and call it `main.rs` Rust files always end with the `.rs` extension. If you are using more than one word in your filename, use an underscore to separate them (e.g `hello_world.rs` instead of `helloworld.rs`).

Now open the `main.rs` file a enter the next code:

```rust
fn main() {
	println("Hello, world!");
}
```

Save the file and go back to your terminal window. On Linux or macOS, enter the following commands to compile and run the file:

```
$ rustc main.rs
$ ./main
Hello, world!
```

Regardless of your operating system, the string `Hello, world!` should print to the terminal. If you do not see this output, refer vack to the [Troubleshooting](https://doc.rust-lang.org/book/ch01-01-installation.html#troubleshooting) part of the installation section for ways to help.

If `Hello, world!` did print, congratulation! You have officially written a Rust program. That makes you a Rust programmer, welcome!

### Anatomy of a Rust Program
Let's view in detail what just happened in your "Hello, world!" program. Here is the first piece of the puzzle.

```rust
fn main() {

}
```

These lines define a function in Rust. The `main` function is special: It is always the first code that runs in every executable Rust program. The first line declares a function named `main` that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses, `()`.

Also, note that the function body is wrapped in curly brackets `{}`. Rust requires these around all function bodies. It is good style to place the opening curly brackets on the same line as the function declaration, adding one space in between. 

> Note: At the time of this writing, an automatic formatter tool called `rustfmt` is under development. If you want to stick to a standard style across Rust projects, `rustfmt` will format your code in a particular style. The Rust team plans to eventually include this tool with the standard Rust distribution, like `rustc`. So depending on when you read this book, it might already be installed on your computer! Check the online documentation for more details.

Inside the `main` function is the following code:

```rust
    println!("Hello, world!");
```

This line does all the work in this little program: It prints text on the screen. There are four important details to notice here:

1. Rust style is to indent with four spaces, not a tab.
2. `println!` calls a Rust macro. If it called a function instead, it would be entered as `println` (without the `!`). We will discuss Rust macros in chapter 19. For now, you just need to know that using `!` means that you are calling a macro instead of a normal function.
3. You see the `"Hello, world!"` string. We pass this string as an argument to `println!`, and the string is printed to the screen.
4. We end the line with a semicolon (`;`), which indicates that this expression is over and the next one is ready to begin. Most of the lines of Rust code end with a semicolon.

### Compiling and Running are Separate Steps
You have just run a newly created program, so let is examine each step in the process.

Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc` command and passing it he name of your source file, like this:

```
$ rustc main.rs
```

> Note: If you have C or C++ background, you will notice that this is similar to `gcc` or `clang`.

After compiling successfully, Rust outputs a binary executable.

On Linux, macOS, and PowerShell on Windows, you can see the executable by entering the `ls` command in your shell. Below the output of the command.

```
$ ls
main  main.rs 
```

The `main.rs` file is the source code file and the `main`is the executable file. From here, you run the `main` file like this:

```
$ ./main
```

If `main.rs` was your "Hello, world!" program, this line print `Hello, world!` to your terminal.

If you are more familiar with a dynamic language (e.g Ruby, Python or JavaScript), you might not be to used to compiling and running a program as separated steps. *Rust is an _ahead-of-time compiled_ language*, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a `.rb` `.py`or `.js` file, the need to have a Ruby, Python, or JavaScript implementation installed. But, in those language, you only need one command to compile and run your program. _Everything is a trade-off in language design_.

Just compiling with `rustc` is fine for simple programs, but as your project grows, you will want to manage all the options an make it easy to share your code. Next, we will introduce you to the *Cargo* tool, which help you write real-world Rust programs.


## 1.3. Hello, Cargo!
Cargo, is Rust's build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you. such as building your code, downloading the libraries your code depends on, and building those libraries.

The simplest Rust programs, like the one we have written so far, do not have any dependencies. so if we had built the "Hello, world!" with Cargo, it would only use the part of Cargo that handles building your code. As your write more complex Rust programs, you will add dependencies and if you start a project using Cargo, adding dependencies will be much easier to do.

Cargo comes installed with Rust if you used the official installed discussed in the [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html#installation) section. If you installed Rust through some other means, check whether Cargo is installed by entering the following in your terminal.

```
$ cargo --version
```

If you see a version number, you have it! If you see an error, such as `command not found` look at the documentation for your method installation to determine how to install Cargo separately.

### Creating a Project with Cargo
Let's create a new project using Cargo and look at how it differs from our original "Hello, world!" project. Navigate back to your `/projects` directory, and then on any operating system run:

```
$ cargo new hello_cargo
$ cd hello_cargo
```

The first command creates a new directory called `/hello_cargo`. We have named our project our project `hello_cargo`, and Cargo creates its files in a directory of the same name.

Go into `/hello_cargo` and list the files. You will see that Cargo has generated two files and one directory for us:

+ `Cargo.toml`
+ `src/main.rs`

It has also initialized a new Git repository along with a `.gitignore` file.

> Note: Git is a common version control system. You can change `cargo new` to use a different version contros system or no version control system by using the `--vcs` flag. Run `cargo new --help` to see the available options.

Open `Cargo.toml` in your text editor it should look like:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]

```

This file is [TOML](https://github.com/toml-lang/toml) (Tom's Obvious Minimal Language) format, which is Cargo's configuration format.

The first line, `[package]` is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we will add other sections.

The next four lines set the configuration information Cargo needs to compile your program. Cargo gets your name and email information from your environment, so if that information is not correct, fix the information now and then save the file.

The last line, `[dependencies]`, is the start of a section for you to list any of your project's dependencies. In Rust, packages of code are referred to as *crates*. We won't need any other crate for this project, but we will in the first project in the dependencies section of the next chapter.

Now open `src/main.rs`. Cargo has generated a "Hello, world!" program for you, just like the one we wrote before. So far, the differences between our previous project and the project Cargo generates are the Cargo placed the code in the `/src` directory, and we have the `Cargo.toml`configuration file in the top directory.

Cargo expects your source files to live inside the `src` directory. The top-level project directory is just for `README` files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There is a place for everything, and everything is in its place.

If you started a project that does not use Cargo, as we did with the "Hello, world!" project, you can convert it to a project that does use Cargo. Move the project code into the `/src`  directory and create an appropiate `Cargo.toml` file.

### Building and Running a Cargo Project
Now let's look at what is different when we build and run the "Hello, world!" program with Cargo. From your `hello_cargo` directory, build your project by entering the following command:

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates and executable file in `target/debug/hello_cargo` rather than in your current directory. You can run the executable with this command:

```
$ ./target/debug/hello_cargo
```

If all goes well, `"Hello, world!"` should be print to the terminal. 

Running `cargo build` for the first time also causes Cargo to create a new file at top level: `Cargo.lock`. This file keeps track of the exact version of dependencies in your project. This project does not have dependencies, so the file is a bit sparse. You won't ever need to change this file manually, Cargo manages its content for you.


Also, we can use `cargo run` to compile the code and then run the resulting executable all in one command

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

This time Cargo figured out that the files had not changed, so it just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it, and you would have see this output.

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compile but does not produce an executable.

```
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want and executable? Often, `cargo check`is much faster than `cargo build`, because it skips the step of producing an executable. If you are continually checking you work while writing the code, using `cargo check`will speed up the process! As such, many Rustaceans run `cargo check`periodically as they write their program to make sure it compiles. Then they run `cargo build` when they are ready to use the executable.

Let's recap what we learned so far about Cargo:
+ We can build a project using `cargo build` or `cargo check`
+ We can build and run a project in one step using `cargo run`
+ Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug` directory.

An additional advantage of using Cargo is that the command are the same no matter which operating system you are working on. So, at this point, we will no longer provide specific instructions for Linux, macOS versus Windows.

### Building for Release
When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create and executable in `target/release/` directory. The optimization make your Rust code run faster, but turning them on lengthens the time it takes for you program to compile.

This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you will give to a user that won't be rebuilt repeatedly and that will run as a fast as possible. If you are benchmarking your code's running time, be sure to run `cargo build --release` and benchmark with the executables in `target/release`.

### Cargo as Convention
Wit simple project, Cargo does not provide a lot of value over just using `rustc`, but it will prove its worth as your programs become more intricate. With complex projects composed of multiple crates, it is much easier to let Cargo coordinate the build.

Even though the `hello_cargo` project is simple, it now uses much of the real tooling, you will use in the rest of your Rust career. In fact, to work on any existing projects, you cans use the following commands to check out the code using Git, change to that project's directory, and build:

```
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

### Links
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

### Summary
You are already off to a great start on your Rust journey! In this chapter you have learned how to:

+ Install the latest stable version of Rust using `rustup`
+ Update to newer Rust version
+ Open locally installed documentation
+ Write and run "Hello, world!" program using `rustc` directly.
+ Create and run a new project using the conventions of Cargo.
