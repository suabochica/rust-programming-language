# Getting Started
Let's start Rust Journey!. There is a lot to learn but every journey starts somewhere. In this chapter we will discuss.

+ Installing Rust on Linux, macOS and Windows.
+ Writing a program that prints `Hello, world!`
+ Using `cargo`, Rusts's packages manager and build.

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

```rs
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

```rs
fn main() {

}
```

These lines define a function in Rust. The `main` function is special: It is always the first code that runs in every executable Rust program. The first line declares a function named `main` that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses, `()`.

Also, note that the function body is wrapped in curly brackets `{}`. Rust requires these around all function bodies. It is good style to place the opening curly brackets on the same line as the function declaration, adding one space in between. 

> Note: At the time of this writing, an automatic formatter tool called `rustfmt` is under development. If you want to stick to a standard style across Rust projects, `rustfmt` will format your code in a particular style. The Rust team plans to eventually include this tool with the standard Rust distribution, like `rustc`. So depending on when you read this book, it might already be installed on your computer! Check the online documentation for more details.

Inside the `main` function is the following code:

```rs
    println!("Hello, world!");
```

This line does all the work in this little program: It prints text on the screen. There are four important details to notice here:

1. Rust style is to indent with four spaces, not a tab.
2. `println!` calls a Rust macro. If it called a function instead, it would be entered as `println` (without the `!`). We will discuss Rust macros in chapter 19. For now, you just need to know that using `!` means that you are calling a macro instead of a normal function.
3. You see the `"Hello, world!"` string. We pass this string as an argument to `println!`, and the string is printed to the screen.
4. We end the line with a semicolon (`;`), which indicates that this expression is over and the next one is ready to begin. Most of the lines of Rust code end with a semicolon.

## 1.3. Hello, Cargo!
