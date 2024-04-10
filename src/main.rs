use ndarray::{array,Array, Ix2, Axis, ArrayView};
use std::collections::HashSet;

fn main() {
    let a = array![[0,0,0,0,0,0,0,0,0],
    [1,2,3,4,5,6,0,0,0],
    [0,0,0,0,0,0,7,8,9],
    [0,0,0,0,0,4,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0]];
  
    //println!("{}", puzzle_valid(&a));
}
fn row_valid(arr: &Array<i32, Ix2>, check_row: usize) -> bool{
    let row_set: Vec<i32> = arr.row(check_row).to_vec().into_iter().collect();
    match row_set.len(){
        9 => true,
        _ => false
    }
}
fn column_valid(arr: &Array<i32, Ix2>, check_col: usize) -> bool{
    let col_set: Vec<i32> = arr.column(check_col).to_vec().into_iter().collect();
    match col_set.len(){
        9 => true,
        _ => false
    }
}
/*fn square_valid(arr: &ArrayView<'a, &i32, Ix2>, check_sq: u8) -> bool{
    let check_h = (check_sq - 1) - ((check_sq - 1) % 3);
    let (split_above, split_below) = arr.split_at(Axis(0), check_sq-1);
    true
}*/
fn get_square<'a>(arr: &ArrayView<'a, i32, Ix2>, sq: usize) -> Vec<i32>{
    let (above, below) = arr.split_at(Axis(0), 3);
    let (left, right) = above.split_at(Axis(1), 3);
    left.
    
}