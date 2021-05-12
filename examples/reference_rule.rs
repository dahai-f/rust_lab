use std::ops::Index;

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut v = vec![10, 9, 8];
    let first = &v[9];// v.get(9);
    for i in 0..10 {
        // v.push(i);
    }

    println!("{:?}", first);
}
