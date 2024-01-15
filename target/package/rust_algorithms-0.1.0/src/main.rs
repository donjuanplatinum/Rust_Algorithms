pub mod sort;
pub mod find;
pub mod subarray;
pub mod matrix;
use crate::sort::insert_insort::*;
use crate::matrix::*;
use crate::subarray::*;
use crate::find::*;
fn main() {
    let mut a = [1,2,3,0,5,7,9,8,32,299,100];
    insert_insort(&mut a);
    println!("a is now {:?}", a);
}
