use serde::{Deserialize, Serialize};
use soa_derive::{soa_zip, StructOfArray};

#[derive(StructOfArray)]
#[soa_derive(Serialize, Deserialize)]
struct RigidBody {
    position: i32,
    orientation: i32,
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
