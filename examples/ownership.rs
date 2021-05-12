#[derive(Debug, Clone)]
struct A {
    a: i32,
}

fn main() {
    let a = A { a: 10 };
    {
        let aa = a.clone();
    }
    println!("{:?}", a);
}
