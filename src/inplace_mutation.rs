#![allow(unused)]

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let n = v.get_mut(2).expect("Index should be in range");
    *n += 1;
    println!("{v:?}");

    let mut w = vec![1, 2, 3, 4, 5];
    let m = &mut w[2];
    *m += 1;
    println!("{m:?}");
}
