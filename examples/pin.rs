use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ref: *const String = &self.a;
        unsafe {
            self.get_unchecked_mut().b = self_ref;
        }
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    let mut test1 = std::pin::pin!(test1);
    test1.as_mut().init();
    let mut test2 = Test::new("test2");
    let mut test2 = std::pin::pin!(test2);
    test2.as_mut().init();

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}
