use serde::{Deserialize, Serialize};
use soa_derive::soa_zip;
use std::ops::Range;

struct RigidBody {
    position: i32,
    orientation: i32,
}

#[derive(Serialize, Deserialize)]
struct RigidBodyVec {
    position: Vec<i32>,
    orientation: Vec<i32>,
}

struct RigidBodySliceMut<'a> {
    position: &'a mut [i32],
    orientation: &'a mut [i32],
}

impl RigidBodyVec {
    fn new() -> Self {
        Self {
            position: vec![],
            orientation: vec![],
        }
    }

    fn push(&mut self, value: RigidBody) {
        self.position.push(value.position);
        self.orientation.push(value.orientation);
    }

    fn slice_mut(&mut self, range: Range<usize>) -> RigidBodySliceMut {
        RigidBodySliceMut {
            position: &mut self.position[range.clone()],
            orientation: &mut self.orientation[range],
        }
    }

    fn len(&self) -> usize {
        self.position.len()
    }
}

impl<'a> RigidBodySliceMut<'a> {
    fn swap(&mut self, a: usize, b: usize) {
        self.position.swap(a, b);
        self.orientation.swap(a, b);
    }
}

fn main() {
    let mut rigidbody_vec = RigidBodyVec::new();
    rigidbody_vec.push(RigidBody {
        position: 0,
        orientation: 1,
    });
    rigidbody_vec.push(RigidBody {
        position: 2,
        orientation: 3,
    });

    // Will print:
    //  (0, 1)
    //  (2, 3)
    for (position, orientation) in soa_zip!(&rigidbody_vec, [position, orientation]) {
        println!("{:?}", (position, orientation));
    }

    let mut slice = rigidbody_vec.slice_mut(0..rigidbody_vec.len());
    slice.swap(0, 1);
    // Will print:
    // (2, 3)
    // (0, 1)
    for (position, orientation) in soa_zip!(&rigidbody_vec, [position, orientation]) {
        println!("{:?}", (position, orientation));
    }
}
