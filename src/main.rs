use std::io::{self, BufRead};
use std::time::{Instant};
use std::mem;

fn q_rsqrt(number: f32) -> f32 {
    let mut i : i32;
    let x2: f32;
    let mut y: f32;
    const THREEHALVES: f32 = 1.5;

    x2 = number * 0.5;
    y = number;

    // Evil floating point bit level hacking
    i = unsafe {mem::transmute(y)};
    
    // What the fuck?
    i = 0x5f3759df - (i >> 1);
    y = unsafe {mem::transmute(i)};
    
    // 1st iteration
    y = y * (THREEHALVES - (x2 * y * y));

    // 2nd iteration, this can be removed
    // y = y * (THREEHALVES - (x2 * y * y));
    
    return y;
}

fn main() {
    // Ask for a number
    println!("Enter a number: ");

    // Take input and convert it to an f32 in one beautiful line
    let input_as_float = io::stdin().lock().lines().next().unwrap().unwrap().parse::<f32>().unwrap();
    
    // Start timer, run q_rsqrt function, end timer
    let q_rsqrt_start = Instant::now();
    let q_rsqrt_result = q_rsqrt(input_as_float);
    let q_rsqrt_end = Instant::now();

    // Start timer, do math, end timer
    let standard_rsqrt_start = Instant::now();
    let standard_rsqrt_result = 1.0/(input_as_float.sqrt());
    let standard_rsqrt_end = Instant::now();

    // Print results of q_rsqrt
    println!("\n{}", q_rsqrt_result);
    println!("The fast inverse square root took {:?}", q_rsqrt_end.duration_since(q_rsqrt_start));

    // Print results of the regular way
    println!("\n{}", standard_rsqrt_result);
    println!("The standard 1/sqrt(x) took {:?}", standard_rsqrt_end.duration_since(standard_rsqrt_start));
}
