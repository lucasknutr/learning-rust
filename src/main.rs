use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use gpio_cdev::{Chip, LineRequestFlags};
use std::error::Error;

fn main() {
    let number = sum(23, 12);
    println!("{}", number);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// Learning how to use different types of functions in Rust

fn stringToInteger(str: &str) -> i32 {
    let x = str.parse::<i32>().unwrap();
    x
}

fn integerToString() {
    let x = 123.to_string();
    println!("{}", x);
}

// Is a value without semicolon a return value?
// Yes, it is. The last expression in a function without a semicolon is a return value.
// So, there is not an explicit 'return' keyword in Rust?


fn isFlamengoFirst(){
    let mut flamengo = Team::new("Flamengo");
    let mut vasco = Team::new("Vasco");
    let mut botafogo = Team::new("Botafogo");
    let mut fluminense = Team::new("Fluminense");
    let mut teams = [flamengo, vasco, botafogo, fluminense];
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut fourth = 0;
    for team in teams.iter() {
        if team.points > first {
            fourth = third;
            third = second;
            second = first;
            first = team.points;
        } else if team.points > second {
            fourth = third;
            third = second;
            second = team.points;
        } else if team.points > third {
            fourth = third;
            third = team.points;
        } else if team.points > fourth {
            fourth = team.points;
        }
    }
    println!("{} {} {} {}", first, second, third, fourth);
}

fn sumOfArrays() {
    let mut array1 = [1, 2, 3, 4, 5];
    let mut array2 = [6, 7, 8, 9, 10];
    let mut array3 = [0, 0, 0, 0, 0];
    for i in 0..5 {
        array3[i] = array1[i] + array2[i];
    }
    println!("{:?}", array3);
}

fn testino() {
    let mut array = [1, 2, 3, 4, 5];
    let mut array2 = [0, 0, 0, 0, 0];
    for i in 0..5 {
        array2[i] = array[i] * 2;
    }
    println!("{:?}", array2);
}

