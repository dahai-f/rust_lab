fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}