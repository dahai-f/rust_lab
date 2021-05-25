use std::any::Any;

use metatype::type_coerce;

trait MyTrait1: Any {
    fn fun(&self) {}
}

struct MyStruct1;

impl MyTrait1 for MyStruct1 {}
trait MyTrait2: MyTrait1 {
    fn fun(&self) {}
    fn fun2(&self) {}
}

struct MyStruct3;

struct MyStruct2;

impl MyTrait1 for MyStruct2 {}
impl<T: MyTrait1> MyTrait2 for T {}

fn main() {
    println!("{:?}", std::any::TypeId::of::<dyn MyTrait2>());
    println!("{:?}", std::any::type_name::<dyn MyTrait2>());
    println!("{:?}", std::any::TypeId::of::<MyStruct2>());
    println!("{:?}", std::any::type_name::<MyStruct2>());

    let t = Box::new(MyStruct2) as Box<dyn MyTrait1>;
    let t = &*t as *const dyn MyTrait1;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);
    let t = Box::new(MyStruct2) as Box<dyn MyTrait1>;
    let t = &*t as *const dyn MyTrait1;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct2) as Box<dyn MyTrait2>;
    let t = &*t as *const dyn MyTrait2;
    let trait_object: metatype::TraitObject = type_coerce(metatype::Type::meta(t));
    println!("{}", trait_object.vtable as *const _ as usize);

    let t = Box::new(MyStruct2) as Box<dyn Any>;
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
