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

## 2. Storing UTF-8 Encoded Text with Strings

## 3. Storing Keys with Associated Values in Hash Map
