//------------------------------------
//  Rust Modules
//------------------------------------
/*
use rust_module::file_1;
use rust_module::file_2;

fn main() {
    rust_module::file_1::printing();
    rust_module::file_2::printing();
} 

mod file_1;

fn main() {
    let rect1 = Rectangle {
        length: 5,
        width: 10,
    };

    file_1::rect_area(&rect1.length, &rect1.width);
}

struct Rectangle{
    length: i32,
    width: i32,
}


mod file_2;

fn main() {
    file_2::some_person();
}


mod file_3;

fn main() {
    file_3::allowance();
}
*/

//------------------------------------
//  Using External Crates
//------------------------------------
/*
use array_tool::vec::*;
fn main() {
    let vec_1 = vec![1,2,3,1,5,3,5,2];
    let vec_2 = vec![1,2,3];

    let intersection = vec_1.intersect(vec_2.clone());
    println!("The intersection = {:?}", intersection);

    let union_set = vec_1.union(vec_2.clone());
    println!("The union = {:?}", union_set);

    println!("Vec 2 three times displayed = {:?}", vec_2.times(3));
}


//------------------------------------
//  Publishing Crate
//------------------------------------
fn main() {

}
*/


//------------------------------------
//  Smart Pointers
//    - Box Smart Pointer
//    - Use case of Box Smart Pointer
//    - Custom Defined Smart Pointer
//    - Deref coercion
//------------------------------------

/*
fn main() {
    let single_value = Box::new(0.625);
    let x = 0.625;
    println!("Are the value being equal {}", x == *single_value);

    let mut stack_var = 4;
    let stack_ref = &stack_var;

    let heap_var = Box::new(stack_var);

    stack_var = 5;
    println!("The value of stack_var = {} and heap_var = {}", stack_var, heap_var);

    let point = Box::new((100, 125));
    println!("{} {}", 100 == point.0, point.1);

    let x = *point;

}

#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Some(Box::new(Cons(2, Some(Box::new(Cons(3, None)))))));
    println!("{:?}", list);
}
*/

struct MySmartPointer<T: std::fmt::Debug> {
    value:T
}

impl<T: std::fmt::Debug> MySmartPointer<T> {
    fn new(x:T) -> MySmartPointer<T> {
        MySmartPointer { value: x}
    }
}


use std::ops::Deref;

impl<T: std::fmt::Debug> Deref for MySmartPointer<T>{
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("dropping mysmartpointer object from memory {:?}", self.value);
    }
}

fn my_fn(str: &str) {
    println!("The string received from the main is {}", str);
}

fn main() {
    /*
    let a = 50;
    let b = Box::new(a);
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
    //println!("{}", a == b);

    let sptr1 = MySmartPointer::new(a);
    let sptr2 = MySmartPointer::new(*b);

    println!("{}", a == *sptr1); // *(sptr1.deref())

    // let z = *sptr1;

    drop(sptr1);
    */

    let sptr_1 = MySmartPointer::new("Nouman Azam");
    my_fn(&sptr_1);     // &mysmartpointer -> &String -> &str

    let some_vec = MySmartPointer::new(vec![1,2,3]);

    for z in &*some_vec {
        println!("The value is {}", z);
    }
}