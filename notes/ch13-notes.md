# Functional Language Features: Iterators and Closures #

Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is *functional programming*. Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.

In this chapter, we won’t debate the issue of what functional programming is or isn’t but will instead discuss some features of Rust that are similar to features in many languages often referred to as functional.

More specifically, we’ll cover:

- *Closures*, a function-like construct you can store in a variable
- *Iterators*, a way of processing a series of elements
- How to use these two features to improve the I/O project in Chapter 12
- The performance of these two features (Spoiler alert: they’re faster than you might think!)

Other Rust features, such as pattern matching and enums, which we’ve covered in other chapters, are influenced by the functional style as well. Mastering closures and iterators is an important part of writing idiomatic, fast Rust code, so we’ll devote this entire chapter to them.

## Index ##
1. Closures: Anonymous functions that can capture their environment
2. Processing a series of items with Iterators
3. Improving our I/O Project
4. Comparing Performance: Loops vs Iterators

## Closures: Anonymous functions that can capture their environment ##
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined. We’ll demonstrate how these closure features allow for code reuse and behavior customization.

### Creating an Abstraction Behavior with Closures ###
Let’s work on an example of a situation in which it’s useful to store a closure to be executed later. Along the way, we’ll talk about the syntax of closures, type inference, and traits.

Consider this hypothetical situation: we work at a startup that’s making an app to generate custom exercise workout plans. The backend is written in Rust, and the algorithm that generates the workout plan takes into account many factors, such as the app user’s age, body mass index, exercise preferences, recent workouts, and an intensity number they specify. The actual algorithm used isn’t important in this example; what’s important is that this calculation takes a few seconds. We want to call this algorithm only when we need to and only call it once so we don’t make the user wait more than necessary.

We’ll simulate calling this hypothetical algorithm with the function `simulated_expensive_calculation` shown below, which will print `calculating slowly...`, wait for two seconds, and then return whatever number we passed in.

```rs
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    
    intensity
}
```

Next is the `main` function, which contains the parts of the workout app important for this example. This function represents the code that the app will call when a user asks for a workout plan. Because the interaction with the app’s frontend isn’t relevant to the use of closures, we’ll hardcode values representing inputs to our program and print the outputs.

The required inputs are these:

- An intensity number from the user, which is specified when they request a workout to indicate whether they want a low-intensity workout or a high-intensity workout
- A random number that will generate some variety in the workout plans

The output will be the recommended workout plan.

```rs
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
```

We’ve hardcoded the variable `simulated_user_specified_value` as 10 and the variable `simulated_random_number` as 7 for simplicity’s sake; in an actual program, we’d get the intensity number from the app frontend, and we’d use the `rand` crate to generate a random number, as we did in the Guessing Game example in Chapter 2. The main function calls a `generate_workout` function with the simulated input values.

Now that we have the context, let’s get to the algorithm. The function `generate_workout` contains the business logic of the app that we’re most concerned with in this example. The rest of the code changes in this example will be made to this function.


```rs
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushuops!!",
            simulated_expensive_calculation(intensity)
        )
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

This code has multiple calls to the slow calculation function. The first if block calls `simulated_expensive_calculation` twice, the if inside the outer else doesn’t call it at all, and the code inside the second else case calls it once.

The desired behavior of the `generate_workout` function is to first check whether the user wants a low-intensity workout (indicated by a number less than 25) or a high-intensity workout (a number of 25 or greater).

Low-intensity workout plans will recommend a number of push-ups and sit-ups based on the complex algorithm we’re simulating.

If the user wants a high-intensity workout, there’s some additional logic: if the value of the random number generated by the app happens to be 3, the app will recommend a break and hydration. If not, the user will get a number of minutes of running based on the complex algorithm.

This code works the way the business wants it to now, but let’s say the data science team decides that we need to make some changes to the way we call the `simulated_expensive_calculation` function in the future. To simplify the update when those changes happen, we want to refactor this code so it calls the `simulated_expensive_calculation` function only once. We also want to cut the place where we’re currently unnecessarily calling the function twice without adding any other calls to that function in the process. That is, we don’t want to call it if the result isn’t needed, and we still want to call it only once.

#### Refactoring Using Functions ####
We could restructure the workout program in many ways. First, we’ll try extracting the duplicated call to the `simulated_expensive_calculation` function into a variable, as shown next.

```rs
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity)
    if intensity < 25 {
        println!(
            "Today, do {} pushuops!!",
            expensive_result
        )
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, for {} minutes!",
                expensive_result
            );
        }
    }
}
```

This change unifies all the calls to `simulated_expensive_calculation` and solves the problem of the first `if` block unnecessarily calling the function twice. Unfortunately, we’re now calling this function and waiting for the result in all cases, which includes the inner `if` block that doesn’t use the result value at all.

We want to define code in one place in our program, but only *execute* that code where we actually need the result. This is a use case for closures!

#### Refactoring with Closures to Store Code ####
Instead of always calling the `simulated_expensive_calculation` function before the if blocks, we can define a closure and store the closure in a variable rather than storing the result of the function call, as shown in the next snippet. We can actually move the whole body of `simulated_expensive_calculation` within the closure we’re introducing here.

```rs
let expensive_closure = |num| {
    println!(calculating slowly...);
    thread::sleep(Duration::from_secs(2));
    
    num
};
```

The closure definition comes after the `=` to assign it to the variable `expensive_closure`. To define a closure, we start with a pair of vertical pipes (`|`), inside which we specify the parameters to the closure; this syntax was chosen because of its similarity to closure definitions in Smalltalk and Ruby. This closure has one parameter named `num`: if we had more than one parameter, we would separate them with commas, like `|param1, param2|`.

After the parameters, we place curly brackets that hold the body of the closure—these are optional if the closure body is a single expression. The end of the closure, after the curly brackets, needs a semicolon to complete the let statement. The value returned from the last line in the closure body (`num`) will be the value returned from the closure when it’s called, because that line doesn’t end in a semicolon; just as in function bodies.

Note that this let statement means `expensive_closure` contains the definition of an anonymous function, not the resulting value of calling the anonymous function. Recall that we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in `expensive_closure`.

With the closure defined, we can change the code in the if blocks to call the closure to execute the code and get the resulting value. We call a closure like we do a function: we specify the variable name that holds the closure definition and follow it with parentheses containing the argument values we want to use, as shown below:

```rs
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!(calculating slowly...);
        thread::sleep(Duration::from_secs(2));

        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        )
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, for {} minutes!",
                expensive_result
            );
        }
    }
}
```
Now the expensive calculation is called in only one place, and we’re only executing that code where we need the results.

However, we’ve reintroduced one of the problems from our last approach: we’re still calling the closure twice in the first if block, which will call the expensive code twice and make the user wait twice as long as they need to. We could fix this problem by creating a variable local to that if block to hold the result of calling the closure, but closures provide us with another solution. We’ll talk about that solution in a bit. But first let’s talk about why there aren’t type annotations in the closure definition and the traits involved with closures.

### Closure Types Inference and Annotation ###
### Storing Closures Using Generic Parameters and the Fn Traits ###
### Limitations of the Cacher Implementation ###
### Capturing the Environment with Closures ###

## Processing a series of items with Iterators ##

## Improving our I/O Project ##

## Comparing Performance: Loops vs Iterators ##
