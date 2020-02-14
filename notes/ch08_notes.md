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
New Rustaceans commonly get stuck on strings for a combination of three reasons:

1. Rust's propensity for exposing possible errors.
2. Strings being a more complicated data structure than many programmers give them credit for.
3. UTF-8

These factors combine in a way that can seem difficult when you are coming from ohter programming languages.

It is useful to discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those btes are interpreted as text. In this section, we will talk about the operations on `String` that every collection type has, such as creating, updating, and reading. We will also discuss the rays which `String` is differento from other collections, namely how indexing into a `String` is complicated by the differences between how people and computers interpret `String` data.

### What is a String?
Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed from `&str`. We talked about _string slices_ which are references to some UTF-8 encoded string data stores elsewhere. String literals, for example, are stored in the program's binary and are therefore string slices.

The `String` type, which is provided by Rust's standard library rather than code into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refers to strings in Rust, they usually mean The `String` and the string slice `&str` types, not just one of those types. Although this section is largely about `String`, both types are used heavily in Rust's standard library, and both `String` and string slices are UTF-8 encoded.

Rust's standard library also includes a number of other string types. Library crates can provide even more options for storing string data. They refer to owned and borrowed variants, just like the `String` and `str` types you have seen previously. These string types can store text in different encodings or be represented in memory in a different way, for example. We won't discuss these other string types in this chapte, see their API documentation for more about how to use them and when eac is appropiate.

### Creating a New String
Many of the same operations available with `Vec<T>` are available with `String` as well, starting with the `new` function to create a string, shown below.

```rust
let mut s = String::new();
```

This line creates a new empty string called `s`, which we can then load data into. Often, we will have some initial data that we want to start the string with. For that, we use the `to_string` method, which is available on an any type that implements the `Display` trait, as string literals do. Here we have two examples.

```rust
let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string();
```

This code crates a string containing `initial contents`.

We can also use the function `String::from` to create a `String` from a string literal. The last code is equivalent to the next one:

```rust
let s = String::from("initial contents");
```

Because string are used for so many things, we can use many different generic APIs for strings, providing us wiht a lot of options. Some of them can seem redundant, but they all have their place. In this case, `String::from` and `to_string` do the same thing, so which you choose is a matter of styles.

Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as shown below:

```rust

#![allow(unused_variables)]
fn main() {
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
}
```
 All of these are valid `String` values.

### Updating a String
A `String` can grow in size an its content can change, just like contents of `Vect<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values

#### Appending to a String
We can grow a `String` by using the `push_str` method to append string slice, as shown next:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

After these two lines, `s` will contain `foobar`. The `push_str` method takes a string slice because we do not necessarily want to take ownership of the parameter. For example, in the snippet below, we show that iw would be unfortunate if we were not able to sue `s2` after appenting its contents to `s1`.

```rust
let mut s1 = String::from("foo");
let s2 = "bar";

s1.push_str(s2);
println!("s2 is {}", s2);
```

If the `push_str` method took ownership of `s2`, we would not be able to print its values on the last line. However, this code works as we would expect.

The `push` method takes a single character as a parameter and adds it to the `String`. Next code show how to adds the letter `l` to a `String` using the `push` method.

```rust
let mut s = String::from("lo");
s.push('l');
```

As a result of this code, `s` will contain `lol`.

#### Concatenation
Often, you will want to combine two existing strings. One way is to use the `+` operator, as shown below.

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2 // note s1 has been moved here and can no longer be used.
```

The string `s3` will contain `Hello, world!` as a result of this code. The reason `s1` is no longer valid after the addition and the reason we used a reference to `s2` has to do with the signature of the method that gets called when we use the `+` operator. The `+` operator uses the `add` method, whose signature looks somethin like this:

```rust
fn add(self, s: &str) -> String {}
```

This is no the exact signature that is in the standard library, because there is defined using generics. Here, we are looking at the signature of `add` with concrete types subtitued for the generic ones, whihch is what happens when we call this method with `String` values. This signature gives us the clues we need to understand the tricky bits of the `+` operator.

First, `s2` has an `&`, meaning that we are adding a _reference_ to the second to the first string because of the `s` parameter in the `add` function. We can only add a `&str` to a `String`. We cannot add two `String` values together. But wait, the type of `&s2` is `&String`, not `&str`, as specified in the second parameter to `add`. So this code will compile?

The reason we are able to use `&s2` in the call to `add` is that the compiler can _corce_ the `&String` argument into `&str`. When we call the `add` method, Rust uses a _deref coercion_, which here turns `&s2` into `&s2[..]`. We will discuss more of deref coercion in chapter 15. Because `add` does not take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation.

Second, we can see in the signature that `add` takes ownership of `self`, because `self` does not have an `&`. This means `s1` will be moved into the `add` call and no longer be valid after that. So although `let s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. In other words, it looks like it is making a lot of copies but is not. The implementation is more efficient than copying.

If we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"` characters, it is difficult to see what is going on. For more complicated string combining, we can use the `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

This code also sets `s` to `tic-tac-toe`. the `format!` macro works in the same way as `println!`, but instead of printing the output to the screen, it returns a `String` with the contents. The version of the code using `format!` is much easier to read and does not take ownership of any of its parameters.

### Indexing into String
In many programming languages, accessing to individual characters in a string by referencing the by index is a valid and common operation. However, if you try to access parts of a `String` using indexing syntax in Rust you will get an error.

```rust
let s1 = String::from("hello");
let h = s1[0];
```

This code will result in the following error.

```
error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
 -->
  |
3 |     let h = s1[0];
  |             ^^^^^ the type `std::string::String` cannot be indexed by `{integer}`
  |
  = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
```

The error and the note tell the story: Rust string do not support indexing. Why not? To answer that question we need to discuss how Rust stores strings in memory.

#### Internal Representation
A `String` is a wrapper over a `Vec<u8>`. Let's look at some of our properly encoded UTF-8 examples string from the list that we discuss before:

```rust
let len = String::from("Hola").len();
```

In this case, `len` will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of these letters take 1 byte when encoded in UTF-8. But what about the following line:

```rust
let len = String::from("Здравствуйте").len();
```

Asked how long the string is, you might say 12. However, Rust's answer is 24. That is the number of bytes it takes to encode "Здравствуйте" in UTF-8, beacuse each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar value. To demonstrate, conside this invalid Rust code:

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

What should the values of `answer` be? When encoded in UTF-8, the first byte of `З` is 208 and the second is 151, so `answer` should in the fact be 208, bat 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string. However, that is the only data that Rust has at byte index 0. Users generally do not want the byte returned, even if the string contains only Latin letters. If `&"hello"[0]` were valid code that returned the byte value, it would return 104, not `h`. To avoid returning an unexpected values and causing bugs that might not be sicovered immediately, Rust does not compile this code at all and prevents misunderstandings early in the development process.

#### Bytes, Scalar Values and Grapheme Clusters
Another poing about UTF-8 is that there are actually three relevant ways to look at strings from Rust's perspective:

1. Bytes
2. Scalar values
3. Grapheme clusters a.k.a _letters_

If we look at the Hindi word  word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

That is 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust's `char` type is, those bytes look like this:

```
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters. They are diacritics that do not make sense on their own. Finally, if we look at them as grapheme clusters, we would get what a person would call the four letters that make up the Hindi word.

```
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

A final reason Rust does not allow us to index into a `String` to get a character is that indexing operations are expected to always takes constatn time `0(1)`. But it is not possible to guarantee that performance with a `String`, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

### Slicing String
Indexing into a string is often a bad idea because it is not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices. To be more specific, in your indexing and indicate that you want a string slice, rather that indexing using `[]` with a single number, you can use `[]` with a range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters wes 2 bytes, which means `s` will be `Зд`.

What would happen ig we used `&hello[0..1]`? the answer, Rust would panic at runtime in the same way as if an invalid index were accessed in a vector.

```
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4
```

You should use ranges to create string slices with caution, because doing so can crash your program.

### Iterating Over String
Fortunately, you can access elements in a string in other ways.

If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the `chars` method. Calling `chars` on "नमस्ते" separates out and returns six values of types `char`, and you can iterate over the result to access each elements.

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

This code will print the following:

```
न
म
स
्
त
े
```
The `bytes` method returns each raw byte, which might be appropriate for your domain.

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

This code will print the 18 bytes that make yp this `String`:

```
224
164
// --snip--
165
135
```

But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. Crates are availabe on crate.io if this is the functionality you need.

### String are not Simple
To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of `String` data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings that is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

Let's swith to something a bit less complex: Hash maps.

## 3. Storing Keys with Associated Values in Hash Map
The last of our common collections is the _hash map_. The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`, It does this via a _hashing function_ which determine how it places these keys an values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

Hash maps are useful when you want to look up data not by using and index, as you can with vectors, but by using a key that can be of any type. For example, in a game, you could keep track of each team's score in a hash map in which each key is a team's name and the value are each team's score. Give a team name, you can retrieve its score.

We will go over the basic API of hash maps in this section, but many more goodies are hiding in the functions defined on `HashMap<K, V>` by the standard library. As always, check the standard library documentation for more information.

### Creating a New Hash Map
You can create an empty hash map with `new` and add elements with `insert`. In the next code, we are keeping track of the scores of two teams whose names are Blue and Yellow. The Blue team starts with 10 points, and the Yellow team starts with 50.

```rust
use std::collections:HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Note that we need to first `use` the `HashMap` from the collections portion of the standard library. Of our three common collections, this is the least often used, so it is not included in the features brought into scope automatically in the prelude. Hash maps are also have less support from the standard library; there is no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This `HashMap` has keys of type `String` and values of types `i32`. Like vectors, hash maps are homogeneous. All of the keys must have the same type, and all the values must have the same type.

Another way of constructing a hash map is by using the `collect` method on a vector of tuples, where each tuple consists of a key and its value. The `collect` method gathers data into a number of collection types, including `HasMap`. For example, if we had the team names and initial scores in two separate vectors, we could use `zip` method to create a vector of tuples where "Blue" is paired with 10, and so forth. Then we could use the `collect` method to turn that vector of tuples into a hash map, as shown below.

```rust
use std::collection::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> =  teams.iter().zip(initial_scores.iter()).collect();
```

The type annotation `HashMap<_,_>` is needed because it is possible to `collect` into many different data structures and Rust does not know which you want unless you specify. For the parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

### Hash Map and Ownership
For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values likes `String`, the values will moved and the hash map will be the owner of those values, as demonstrated here:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
```

We are not able to use the variables `field_name` and `field_value` after they have been moved into the hash map with the call to `insert`.

If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the references point must be valid for at least as long as the hash map is valid. We will talk more about these issue in the "Validating References with Lifetimes" section in chapter 10.

### Accessing Values in a Hash Map
We can get a values out of the hash map by providing its key to the `get` method as shown the next snippet.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name)
```

Here, `score` will have the value that is associated with the Blue team, and the result will be `Some(&10)`. The result is wrapped in `Some` because `get` returns an `Option<&V>`. If there is no value for that key in the hash map, `get` will return `None`. The program will need to handle the `Option` in one of the ways we covered in chapter 6.

We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a `for` loop:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
This code will print each pair in an arbitrary order:

```
Yellow: 50,
Blue: 10,
```

### Updating a Hash Map
Although the number of keys and values is growable, each key can only have one value associated with it at a time. When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key _does not_ already have a value. Or you could combine the old value and the new value. Let's look at how to do each of these.

#### Overwriting a Value
If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced. Even though the code below calls `insert` twice, the hash map will only contain one key/value pair because we are inserting the value for the Blue team's key both times. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:>}", scores)
```

This code will print `{"Blue": 25}`. The original value of `10` has been overwritten.

#### Only Inserting a Value if the Key Has No Value
It is common to check whether a particular key has a value and, if it does not, insert a value for it. Hash map have a special API for this called `entry` that takes the key you want to check as a parameter. The return values of the `entry` method is an enum called `Entry` that represents a value that might or might not exist. Let's say we want to check whether the key for the Yellow team has a value associated with it. If it does not, we want to insert the value 50, and the same for the Blue team. Using the `entry` API, the code look like:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:>}", scores)
```

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner that writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-25 will print `{"Yellow": 50, "Blue": 10}`. The first call to `entry` will insert the key for the Yellow team with the value 50 because the Yellow team does not have a value already. The second call to `entry` will not change the hash map because the Blue team already has the value 10.

#### Updating a Value Based on the Old Value

### Hashing Functions
### Summary
