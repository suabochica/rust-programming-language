# Chapter 5: Using Structs to Structure Related Data
A _struct_ or _structure_ is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you are familiar with object-oriented language, a _struct_ is like an object's data attribute. In this chapter, we will compare and contrast tuples with structs, demonstrate how to use structs, and discuss how to define methods and associated functions to specify behavior associated with a struct's data. Structs and enums are the building blocks for creating new types in your program's domain to take full advantage of Rust's compile time type checking.


## Index
1. Defining and Instantiating Structs
2. An Example Program Using Structs
3. Method Syntax

## 1. Defining and Instantiating Structs
Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you will name each piece of data so it is clear what the values mean. As a result of these names, structs are more flexible than tuples. You do not have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A struct's name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call _fields_. For example, the next snippet shows a struct that stores information about a user account.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
}
```

To use a struct after we have defined it, we create an _instance_ of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing `key: value` pairs, where the keys are the names of the fields and the values are data we want to store in those fields. We do not have to specify the fields in the same order in which we declared them in the struct. In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type. For example, we can declare a particular user as shown the code below.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };
}
```

To get a specific value from a struct, we can use dot notation. If we wanted just this user's email address, we could use `user1.email` whenever we wanted to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Next code shows how to change the value in the `email` field of a mutable `User` instance.

```rust
#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
}
```

Note that the entire instance must be mutable. Rust does not allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

Next example shows a `build_user` function that returns `User` instance with the given email and username. The `active` field gets the value of `true`, and the `sign_in_count` gets a value of `1`.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  fn build_user(email: String, username: String) -> User {
      User {
          email: email,
          username: username,
          active: true,
          sign_in_count: 1,
      }
  }
}
```

It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the `email` and `username` field names and variables is a bit tedious. If the struct had more fields, repeating each name would get even more annoying. Luckily, there is a convenient shorthand.

### Using the Field Init Shorthand when Variables and Fields Have the Same Name
Because the parameter names and the struct field names are exactly the same, we can use the _field init shorthand_ syntax to rewrite `build_user`  so that it behaves exactly the same but does not have the repetition of `email` and `username`, as shown below.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  fn build_user(email: String, username: String) -> User {
      User {
          email,
          username,
          active: true,
          sign_in_count: 1,
      }
  }
}
```
Here, we are creating a new instance of the `User` struct, which has a field named `email`. We want to set the `email` fields's value to the value in the email parameter of the `build_user` function. Because the `email` field an the email parameter have the same name, we only need to write `email` rather than `email:email`.

### Creating Instance From Other Instances With Struct Update Syntax
It is often useful to create a new instance of a struct that uses most of an old instance's values but changes some. You will do this using _struct update syntax_.

First, the next snippet shows how we create a new `User` instance in `user2` without the update syntax. We set new values for `email` and `username` but otherwise use the same values from `user` that we created before.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };

  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),
      active: user1.active,
      sign_in_count: user1.sign_in_count,
  };
}
```
Using struct update syntax, we can achieve the same effect with less code, as shown in the next sample. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```rust
#![allow(unused_variables)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };

  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),
      ..user1
  };
}
```

The last code also creates an instance in `user2` that has a different value for `email` and `username` but has the same values for the `active` and `sign_in_count` fields from `user1`.

### Using Tuple Strutcs Without Named Fields to Create Different Types

### Unit-Like Structs Without Any Fields

## 2. An Example Program Using Structs

## 3. Method Syntax
