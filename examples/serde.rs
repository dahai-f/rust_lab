#![feature(arbitrary_self_types)]

use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use lazy_static::*;
use serde::de::DeserializeOwned;
use serde::ser::SerializeTuple;
use serde::*;
use serde_json::Value;

trait DeTrait {
    fn de<T: ?Sized + DeTrait>() -> (&'static str, DeFn<T>)
    where
        Self: Sized,
        Self: DeserializeOwned,
    {
        let sized_type = std::any::type_name::<Self>();
        (sized_type, |value| {
            let boxed = Box::<Self>::new(serde_json::from_value::<Self>(value).unwrap());
            SerdeBox(metatype::type_coerce(boxed))
        })
    }
}

trait SerTrait {
    fn ser(&self) -> Value;
}

impl<T: 'static + Serialize + Sized> SerTrait for T {
    fn ser(&self) -> Value {
        serde_json::to_value(self).unwrap()
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

trait NameToDeTableTrait {
    type DynType: ?Sized + DeTrait;
    fn get_de(sized_type: &str) -> DeFn<Self::DynType>;
}

struct NameToDeTable<T: ?Sized + DeTrait>(PhantomData<T>);

impl NameToDeTableTrait for NameToDeTable<dyn MyTrait> {
    type DynType = dyn MyTrait;

    fn get_de(sized_type: &str) -> fn(Value) -> SerdeBox<Self::DynType> {
        lazy_static! {
            static ref NAME_TO_DE: NameToDe<dyn MyTrait> = {
                let mut map = NameToDe::new();
                let (key, de) = MyStruct::de::<dyn MyTrait>();
                map.insert(key, de);
                let (key, de) = MyStruct1::de::<dyn MyTrait>();
                map.insert(key, de);
                map
            };
        }

        *NAME_TO_DE.get(sized_type).unwrap()
    }
}

type DeFn<T: ?Sized + DeTrait> = fn(value: Value) -> SerdeBox<T>;
type NameToDe<T: ?Sized + DeTrait> = HashMap<&'static str, DeFn<T>>;

#[derive(Debug)]
struct SerdeBox<T: ?Sized>(Box<T>);

impl<T: ?Sized + SerTrait> Serialize for SerdeBox<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut tuple: <S as Serializer>::SerializeTuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(std::any::type_name::<T>())?;
        tuple.serialize_element(&self.0.ser())?;
        tuple.end()
    }
}

impl<'de, T: ?Sized + DeTrait> Deserialize<'de> for SerdeBox<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use std::*;

        struct Visitor<T>(PhantomData<T>);
        impl<T> Visitor<T> {
            fn new() -> Self {
                Self(PhantomData::<T>)
            }
        }
        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> {
            type Value = SerdeBox<T>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a MyTraitBox")
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let sized_type: String = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(serde::de::Error::invalid_length(0, &self)),
                };
                let t1: Value = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(serde::de::Error::invalid_length(1, &self)),
                };

                let de = NameToDeTable::<T>::
                    .get(std::any::type_name::<T>())
                    .unwrap()
                    .get(&sized_type)
                    .unwrap();
                Ok(de(t1))
            }
        }
        deserializer.deserialize_tuple(2, Visitor::new())
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
