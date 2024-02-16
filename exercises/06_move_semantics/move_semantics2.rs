// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66]; // vec0's owner is the main func

    let vec1 = fill_vec(&vec0); // trying to take ownership

    assert_eq!(vec0, vec![22, 44, 66]);  // this isn't possible because vec0 ended after fill_vec took it!
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut newvec: Vec<i32> = vec.clone();

    newvec.push(88);

    newvec.to_vec()
}
