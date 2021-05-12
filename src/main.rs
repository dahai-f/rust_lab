fn main() {
    use serde::*;
    use serde_traitobject as s;
    use std::any::Any;

    #[derive(Serialize, Deserialize, Debug)]
    struct MyStruct {
        foo: String,
        bar: usize,
    }

    trait MyTrait: Any {}

    impl MyTrait for MyStruct {}

    let my_struct = MyStruct {
        foo: String::from("abc"),
        bar: 123,
    };

    let erased: s::Box<dyn s::Any> = s::Box::new(my_struct);

    let serialized = serde_json::to_string(&erased).unwrap();
    let deserialized: s::Box<dyn MyTrait> = serde_json::from_str(&serialized).unwrap();

    let downcast: Box<MyStruct> = Box::<dyn Any>::downcast(deserialized.into_any()).unwrap();

    println!("{:?}", downcast);
    // MyStruct { foo: "abc", bar: 123 }
}
