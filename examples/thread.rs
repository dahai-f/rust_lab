#![feature(negative_impls)]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

struct S {
    value: i32,
}

// impl !Send for S {}
impl !Sync for S {}

fn do_f(f: impl FnOnce()) {
    f()
}

fn main() {
    let mut s = S { value: 10 };
    do_f(|| s.value = 100);
    println!("{}", s.value);

    // let rw_s = Arc::new(RwLock::new(s));
    let t1 = thread::spawn(move || {
        println!("{}", s.value);

        for i in 1..10 {
            println!("Greeting {} from other thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Greeting {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    t1.join().unwrap();
}
