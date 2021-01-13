use std::thread;
use std::time::Duration;

fn main() {
    let t1 = thread::spawn(|| {
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
