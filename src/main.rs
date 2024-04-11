use ndarray::{array,Array, Ix2, Axis, ArrayView};

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
fn get_row(arr: &Array<i32, Ix2>, row: usize) -> Vec<i32>{
    arr.row(row).to_vec().into_iter().collect()
}
fn get_col(arr: &Array<i32, Ix2>, col: usize) -> Vec<i32>{
    arr.column(col).to_vec().into_iter().collect()
}
/*fn square_valid(arr: &ArrayView<'a, &i32, Ix2>, check_sq: u8) -> bool{
    let check_h = (check_sq - 1) - ((check_sq - 1) % 3);
    let (split_above, split_below) = arr.split_at(Axis(0), check_sq-1);
    true
}*/
fn get_square<'a>(arr: &ArrayView<'a, i32, Ix2>, sq: usize) -> Vec<i32>{
    sq = sq - 1;
    let vt = ((sq) % 3) * 3;
    let hz = sq - (vt / 3);
    let (_above, below) = arr.split_at(Axis(0), hz);
    let (after, _discard) = below.split_at(Axis(0), 3);
    let (_left, right) = after.split_at(Axis(1), vt);
    let (final, _discard) =  right.split_at(Axis(1), 3);
    let x = Array::from_iter(final.iter()).to_vec().iter().map(|x|**x).collect();
    x
}