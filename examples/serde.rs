#![feature(arbitrary_self_types)]

use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use lazy_static::*;
use serde::de::DeserializeOwned;
use serde::ser::SerializeTuple;
use serde::*;
use serde_json::Value;

trait SerdeKeyTrait {
    fn serde_key() -> SerdeKey
    where
        Self: 'static + Sized,
    {
        let type_id = std::any::TypeId::of::<Self>();
        let mut hasher = DefaultHasher::default();
        type_id.hash(&mut hasher);
        let type_id = hasher.finish();
        type_id
    }
}

impl<T: 'static + Sized> SerdeKeyTrait for T {}

trait DeTrait: SerdeKeyTrait {
    fn de() -> (SerdeKey, DeFn)
    where
        Self: 'static + Sized,
        Self: DeserializeOwned,
    {
        let type_id = Self::serde_key();
        (type_id, |value| {
            Box::new(serde_json::from_value::<Self>(value).unwrap())
        })
    }
}

trait SerTrait: SerdeKeyTrait {
    fn ser(&self) -> (SerdeKey, Value);
}

impl<T: 'static + Serialize + Sized> SerTrait for T {
    fn ser(&self) -> (u64, Value) {
        (Self::serde_key(), serde_json::to_value(self).unwrap())
    }
}
impl<T: 'static + DeserializeOwned + Sized> DeTrait for T {}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    foo: String,
    bar: usize,
}

trait MyTrait: Debug + SerTrait + DeTrait {
    fn change(&mut self, new_value: usize);
}

impl MyTrait for MyStruct {
    fn change(&mut self, new_value: usize) {
        self.bar = new_value + new_value;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct1 {
    value: usize,
}

impl MyTrait for MyStruct1 {
    fn change(&mut self, new_value: usize) {
        self.value = new_value;
    }
}

type SerdeKey = u64;
type DeFn = fn(value: Value) -> Box<dyn Any>;
type NameToDe = HashMap<SerdeKey, DeFn>;

lazy_static! {
    static ref NAME_TO_DE: NameToDe = {
        let mut map = NameToDe::new();
        let (key, de) = MyStruct::de();
        map.insert(key, de);
        let (key, de) = MyStruct1::de();
        map.insert(key, de);
        map
    };
}

#[derive(Debug)]
struct SerdeBox<T: ?Sized>(Box<T>);

impl<T: ?Sized> SerdeBox<T> {
    fn new(value: Box<T>) -> Self {
        Self(value)
    }
}

impl<T: ?Sized + SerTrait> Serialize for SerdeBox<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut tuple: <S as Serializer>::SerializeTuple = serializer.serialize_tuple(2)?;
        let (type_name, value) = self.0.ser();
        tuple.serialize_element(&type_name)?;
        tuple.serialize_element(&value)?;
        tuple.end()
    }
}

impl<'de, T: ?Sized + DeTrait> Deserialize<'de> for SerdeBox<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use std::*;

        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<dyn Any>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a MyTraitBox")
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let t0: SerdeKey = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(serde::de::Error::invalid_length(0, &self)),
                };
                let t1: Value = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(serde::de::Error::invalid_length(1, &self)),
                };

                let de = NAME_TO_DE.get(&t0).unwrap();
                Ok(de(t1))
            }
        }
        deserializer
            .deserialize_tuple(2, Visitor)
            .map(|any| SerdeBox::new(<boxed::Box<dyn Any> as Into<Box<T>>>::into(any)))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    s: Vec<SerdeBox<dyn MyTrait>>,
}

fn main() {
    let message = Message {
        s: vec![
            SerdeBox::<dyn MyTrait>::new(Box::new(MyStruct {
                foo: String::from("a"),
                bar: 1,
            })),
            SerdeBox::<dyn MyTrait>::new(Box::new(MyStruct1 { value: 2 })),
            SerdeBox::<dyn MyTrait>::new(Box::new(MyStruct1 { value: 3 })),
            SerdeBox::<dyn MyTrait>::new(Box::new(MyStruct {
                foo: String::from("d"),
                bar: 4,
            })),
        ],
    };

    let serialized = serde_json::to_string(&message).unwrap();
    println!("{}", serialized);
    std::fs::write("./resources/message.json", serialized).unwrap();
    // let message = std::fs::read("./resources/message.json").unwrap();
    // let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    // println!("{:?}", deserialized);
    // MyStruct { foo: "abc", bar: 123 }
}
