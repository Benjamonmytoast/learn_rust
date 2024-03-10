mod elevator_events;

use elevator_events::elevator_events;
use memoize::memoize;

fn main() {
    let greeting = greet("Ben", true);
    println!("Greeting is {greeting}");
    println!("{}", fib(35));
    println!("{}", collatz_length(13));
    matrix_exercise();
    geometry_exercise();
    elevator_events();
}

fn greet(name: &str, is_intro: bool) -> String {
    let intro = "Hi guys";
    let outro = "Bye then";
    let prefix = if is_intro { intro } else { outro };
    return format!("{prefix}, it's {name}")
}

/**
* Will calculate the nth Fibonacci number.
* Indices start from 0.
*/#[memoize]
fn fib(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn collatz_odd_even(n: u32) -> u32 {
    match n % 2 {
        0 =>  n/2,
        _ => 3*n+1
    }
}
fn collatz_length(mut n: u32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = collatz_odd_even(n);
        len+=1;
    }
    return len;
}

fn original_collatz_length(n:u32) -> u32 {
    return 1 + match n {
        0|1 => 0,
        _ => collatz_length(collatz_odd_even(n))
    }
}

type Matrix = [[u32; 3];  3];
fn transpose_matrix(matrix: Matrix) -> Matrix{
    let mut transposed: Matrix = [[0; 3]; 3];
    for row_idx in 0..3 {
        for col_idx in 0..3 {
            transposed[col_idx][row_idx] = matrix[row_idx][col_idx];
        };
    };
    return transposed;
}

fn matrix_exercise() {
    let matrix:Matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("matrix: {:#?}", matrix);
    let transposed = transpose_matrix(matrix);
    println!("transposed: {:#?}", transposed);
}

fn magnitude(vector : &[f64; 3]) -> f64 {
    (*vector).iter().fold(0f64, |square_total, cell| square_total + cell.powi(2)).sqrt()
}

fn normalize(vector: &mut[f64; 3]){
    let magnitude = magnitude(vector);
    for fp in vector {
        *fp /= magnitude;
    }
}
fn geometry_exercise() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}