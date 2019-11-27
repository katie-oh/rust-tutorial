use std::io::prelude::*;
use std::io;
use std::fs;

fn main() {
    /****************** Hello World *******************/
    // println!("Hello, world!");

    // // {} crab pinchers

    // // Display formatter
    // println!("Hello, {}", "alice");

    // // Debug formatter
    // println!("Hello, {:?}", "bob");

    /****************** Variables/Mutability *******************/

    // Makes age mutable
    // let mut age = 21;
    // age += 1;

    // println!("{:?}", age);

    /****************** If/Else and Match *******************/

    // println!("{} -> {}", 10, fibonacci(10));

    /****************** Vectors *******************/

    // let mut scores = vec![100, 90, 85];
    // scores[0] -= 10;
    // scores.push(100);
    // println!("scores: {:?}", scores);

    // // Loops over a reference to the vector
    // for score in &scores {
    //     println!("score: {}", score);
    // }

    // println!("Break");

    // // The for loop has ownership over the vector
    // // "Gives" it back piece by piece with "score"
    // // Loops over the vector
    // // scores is now gone from memory
    // for score in scores {
    //     println!("score: {}", score);
    // }

    /****************** Loops *******************/

    // let mut scores = vec![100, 90, 85];
    // // Loops over a reference to the vector
    // for score in &scores {
    //     println!("score: {}", score);
    // }

    // println!("Break");

    // // The for loop has ownership over the vector
    // // "Gives" it back piece by piece with "score"
    // // Loops over the vector
    // // scores is now gone from memory
    // for score in scores {
    //     println!("score: {}", score);
    // }

    /****************** Iterators *******************/

    // for i in 0..10 {
    //     println!("{}", i);
    // }

    // println!("Break");

    // for i in (0..10).map(|x| x * 2) {
    //     println!("{}", i);
    // }

    // println!("Break");

    // for i in (0..10).filter(|y| y % 2 == 0).map(|x| x * 2) {
    //     println!("{}", i);
    // }

    // Turns iterator into a collection
    // let values: Vec<i32> = (0..10).collect();
   
    // let values: i32 = (0..10).sum();
    // println!("{}", values);

    /****************** Exercise *******************/

    // for i in (0..100).filter(|x| x % 21 == 0) {
    //     println!("{}", i);
    // }

    // let value: i32 = (0..100).filter(|x| x % 21 == 0).sum();
    // println!("Here's the sum: {}", value);

    /****************** Structs *******************/

    // let f = Fahrenheit {
    //     temperature: 25.,
    // };

    // let c = f_to_c(f);
    // println!("{}", c.temperature);

    /******************* Methods ******************/

    // let c = Celsius {
    //     temperature: 25.0,
    // };

    // let f = c.c_to_f();

    // println!("F: {:?}", f.temperature);

    /****************** Strings *******************/
    /****************** FizzBuzz! *******************/
    // fizz_buzz();
    // println!("{:?}", fizz_buzz_2(15));

    /****************** User Input *******************/

    // let mut input = String::new();
    // io::stdin().read_line(&mut input)
    //     .expect("Couldn't read from stdin");
    // println!("Read: {:?}", input);

    /******************* Parsing ******************/

    // let a: i32 = "42".parse().expect("Not an integer");
    // Turbofish syntax ::<xxx>
    // let a = "42".parse::<i32>().expect("Not an integer");

    // println!("a: {:?}", a);

    /*************************************/

    /*************************************/

    /*************************************/

    /*************************************/

    /*************************************/

    /*************************************/
    
}

/****************** If/Else and Match *******************/

// fn fibonacci(a: u32) -> u32 {
//     if a == 0 {
//         0
//     } else if a == 1 {
//         1
//     } else {
//         fibonacci(a - 1) + fibonacci(a - 2)
//     }
// }

// fn fibonacci(a: u32) -> u32 {
//     match a {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(a - 1) + fibonacci(a - 2),
//     }
// }
// fn fibonacci(n: u32) -> u32 {
//     // loop through

//     let mut a = 0;
//     let mut b = 1;
//     let mut fib = a + b;

//     for i in 3..n {
//         if i == 0 || i == 1 {

//         }
//     }

//     fib
// }


/******************* Structs ******************/

// #[derive(Debug)]
// struct Fahrenheit {
//     temperature: f64,
// }

// #[derive(Debug)]
// struct Celsius {
//     temperature: f64,
// }
    
// fn f_to_c(f: Fahrenheit) -> f64 {
//     (f.temperature - 32.) / 1.8
// }

// fn f_to_c(f: Fahrenheit) -> Celsius {
//     Celsius {
//         temperature: (f.temperature - 32.) / 1.8,
//     }
// }

/******************* Methods ******************/

// #[derive(Debug)]
// struct Fahrenheit {
//     temperature: f64,
// }

// #[derive(Debug)]
// struct Celsius {
//     temperature: f64,
// }

// impl Celsius {
//     fn c_to_f(&self) -> Fahrenheit {
//         Fahrenheit {
//             temperature: self.temperature * 1.8 + 32.
//         }
//     }
// }

/****************** Strings *******************/

// fn fizz_buzz() {
//     for i in 1..=100 {
//         if i % 15 == 0 {
//             println!("{:?}", "FizzBuzz");
//         }
//         else if i % 3 == 0 {
//             println!("{:?}", "Fizz");
//         }
//         else if i % 5 == 0 {
//             println!("{:?}", "Buzz");
//         }
//         else {
//             println!("{:?}", i);
//         }
//     }
// }

// fn fizz_buzz_2(i: u32) -> String {
//     if i % 15 == 0 {
//         "FizzBuzz".to_string()
//     }
//     else if i % 3 == 0 {
//         "Fizz".to_string()
//     }
//     else if i % 5 == 0 {
//         "Buzz".to_string()
//     }
//     else {
//         i.to_string()
//     }
//     // "Boo".to_string()
// }

// fn fizz_buzz_2(i: u32) -> String {
//     match (i % 3 == 0, i % 5 == 0) {
//         (true, true) => "FizzBuzz".to_string(),
//         (true, false) => "Fizz".to_string(),
//         (false, true) => "Buzz".to_string(),
//         (false, false) => i.to_string(),
//     }
// }

/******************* User Input ******************/


/*************************************/