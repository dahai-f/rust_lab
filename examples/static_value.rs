trait Value {
    fn get_mut() -> &'static mut i32;
}

impl Value for i32 {
    fn get_mut() -> &'static mut i32 {
        static mut V: i32 = 0;
        unsafe { &mut V }
    }
}

impl Value for u32 {
    fn get_mut() -> &'static mut i32 {
        static mut V: i32 = 0;
        unsafe { &mut V }
    }
}

trait MyTrait {}

impl Value for dyn MyTrait {
    fn get_mut() -> &'static mut i32 {
        static mut V: i32 = 0;
        unsafe { &mut V }
    }
}

trait MyTrait2: MyTrait {}

impl Value for dyn MyTrait2 {
    fn get_mut() -> &'static mut i32 {
        static mut V: i32 = 0;
        unsafe { &mut V }
    }
}

struct MyStruct;

impl MyTrait for MyStruct {}

fn main() {
    println!("{}", i32::get_mut());
    *i32::get_mut() += 1;
    println!("{}", i32::get_mut());
    println!("{}", u32::get_mut());
    println!("{}", <(dyn MyTrait) as Value>::get_mut());
    println!("{}", <dyn MyTrait2>::get_mut());
    // println!("{}", MyStruct::get());
}
