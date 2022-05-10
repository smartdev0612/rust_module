/*
pub mod file_1;
pub mod file_2;
*/

//--------------------------------
// Publishing your crate
//--------------------------------
/* 
These lines are not going to be part of documentation
*/

//! # Basic math crate
//! This is a collection of some generally used math functions
//! 
/// Computes a square of an input number
/// 
/// # Examples
/// ```
/// let n = 5;
/// let answer = rust_module::square(n);
/// assert_eq!(25, answer);
/// ```
/// 
/// # Limitations
/// 
/// # Some other section

pub fn square(num: i32) -> i32 {
    num * num
}

/// Computes a cube of an input number
/// 
/// # Examples
/// ```
/// let n = 5;
/// let answer = rust_module::cube(n);
/// assert_eq!(125, answer);
/// ```
/// 
/// # Limitations
/// 
/// # Some other section

pub fn cube(num: i32) -> i32 {
    num * num * num
}