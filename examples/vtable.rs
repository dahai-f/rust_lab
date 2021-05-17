use std::any::Any;

use metatype::type_coerce;

trait MyTrait1 {}

trait MyTrait2: MyTrait1 {}

struct MyStruct;

impl MyTrait1 for MyStruct {}

struct MyStruct1;

impl MyTrait1 for MyStruct1 {}

impl<T: MyTrait1> MyTrait2 for T {}

fn main() {
    let t = Box::new(MyStruct) as Box<dyn MyTrait1>;
    let t = &*t as *const dyn MyTrait1;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct) as Box<dyn MyTrait2>;
    let t = &*t as *const dyn MyTrait2;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct) as Box<dyn Any>;
    let t = &*t as *const dyn Any;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct1) as Box<dyn MyTrait1>;
    let t = &*t as *const dyn MyTrait1;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct1) as Box<dyn MyTrait2>;
    let t = &*t as *const dyn MyTrait2;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct1) as Box<dyn Any>;
    let t = &*t as *const dyn Any;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);
}
