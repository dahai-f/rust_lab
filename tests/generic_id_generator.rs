#![feature(once_cell)]

use std::lazy::{SyncLazy, SyncOnceCell};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};

static mut NEXT: usize = 0;

fn generic_id<T>() -> usize {
    static mut ID: usize = usize::MAX;
    unsafe {
        if ID == usize::MAX {
            ID = NEXT;
            NEXT += 1;
        }

        ID
    }
}

#[test]
fn it_works() {
    assert_eq!(generic_id::<u32>(), 0);
    assert_eq!(generic_id::<i32>(), 1);
}
