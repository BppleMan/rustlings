// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new();

    let (vec0, mut vec1) = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> (&mut Vec<i32>, &mut Vec<i32>) {
    let vec1 = &vec;
    vec1.copy_within(.., 0);

    vec1.push(22);
    vec1.push(44);
    vec1.push(66);

    (vec, *vec1)
}
