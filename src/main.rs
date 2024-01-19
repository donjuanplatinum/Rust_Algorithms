pub mod sort;
pub mod search;
pub mod subarray;
pub mod matrix;

use crate::sort::*;
use crate::matrix::*;
use crate::subarray::*;
fn main() {
    let mut a = [1,2,3,0,5,7,9,8,32,299,100];
    crate::sort::insertion_sort(&mut a);
    println!("a is now {:?}", a);

    let m = (3,[1,2,3,4,5,6,7,8,9]);
    let n = crate::matrix::square_matrix_multiply(m,m);
    println!("matrix n is {:?}", n);

    let q = [-3,1,2,-8,9,0,2,-10,-5,-20,40,-90];
    let c = crate::subarray::merge_max_subarray(&q);
    println!("the max array is {:?}", c);
    let a = [0,1,3,6,124,232,1312312];
    let c = crate::search::binary_search(&a,&7);
    println!("the c at {:?}", c);

    let mut q = [1231,123,1234,125,3,3,56,745,856,0,4,867,1237,4564124,123];
    crate::sort::heap_sort(&mut q);
    println!("q is {:?}", q);
}
