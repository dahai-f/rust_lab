trait TA {
    fn fa(&self) {
        println!("fa");
    }
}

trait TB {
    fn fb(&self) {
        println!("fb")
    }
}

struct S {}
impl TA for S {}

impl<T: TA> TB for T {}

fn main() {
    let s = S {};
    let a: &dyn TA = &s;
    // a.fb(); // error
}
