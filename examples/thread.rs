#![feature(negative_impls)]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Value {
    value: i32,
}

// impl !Send for Value {}
// impl !Sync for Value {}

struct S<'v> {
    value: Value,
    ref_value: &'v Value,
}

static VALUE: Value = Value { value: 100 };

fn main() {
    let mut s = S {
        value: Value { value: 10 },
        ref_value: &VALUE,
    };
    println!("{:?}", s.value);

    // let rw_s = Arc::new(Mutex::new(s));
    let t1 = thread::spawn(move || {
        println!("{:?}", s.value);

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
