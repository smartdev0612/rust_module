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