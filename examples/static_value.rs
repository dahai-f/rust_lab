trait Value {
    fn get() -> i32;
}

impl Value for i32 {
    fn get() -> i32 {
        1
    }
}

impl Value for u32 {
    fn get() -> i32 {
        2
    }
}

trait MyTrait {}

impl Value for dyn MyTrait {
    fn get() -> i32 {
        3
    }
}

trait MyTrait2: MyTrait {}

impl Value for dyn MyTrait2 {
    fn get() -> i32 {
        4
    }
}

struct MyStruct;

impl MyTrait for MyStruct {}

fn main() {
    println!("{}", i32::get());
    println!("{}", u32::get());
    println!("{}", <dyn MyTrait>::get());
    println!("{}", <dyn MyTrait2>::get());
    // println!("{}", MyStruct::get());
}
