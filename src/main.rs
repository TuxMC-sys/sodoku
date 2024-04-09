use ndarray::prelude::*;
use ndarray::Array;
const EXAMPLE_PUZZLE: [[u8;9];9] = [[0,0,0,0,0,0,0,0,0],
    [1,2,3,4,5,6,0,0,0],
    [0,0,0,0,0,0,7,8,9],
    [0,0,0,0,0,4,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0]];
fn main() {
    let test = Array::<u8, Ix2>::zeros((9,9));
    //let column = new_puzzle(EXAMPLE_PUZZLE);
    println!("{:?}",test);
}
