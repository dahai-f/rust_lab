use serde::ser::SerializeTuple;
use serde::*;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    foo: String,
    bar: usize,
}

trait MyTrait: Debug {
    fn change(&mut self, new_value: usize);
    fn ser(&self) -> serde_json::Value;
}

impl MyTrait for MyStruct {
    fn change(&mut self, new_value: usize) {
        self.bar = new_value + new_value;
    }

    fn ser(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}

fn de(value: Value) -> Box<dyn MyTrait> {
    let value: MyStruct = serde_json::from_value(value).unwrap();
    Box::new(value)
}

#[derive(Debug)]
struct MyTraitBox(Box<dyn MyTrait>);

impl MyTraitBox {
    fn new(my_trait: impl 'static + MyTrait) -> Self {
        Self(Box::new(my_trait))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    s: Vec<MyTraitBox>,
}

impl Serialize for MyTraitBox {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut tuple: <S as Serializer>::SerializeTuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(std::any::type_name::<MyStruct>())?;
        tuple.serialize_element(&self.0.ser())?;
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for MyTraitBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use std::*;

        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MyTraitBox;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a MyTraitBox")
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let t0: String = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(serde::de::Error::invalid_length(0, &self)),
                };
                println!("{}", t0);
                let t1: Value = match seq.next_element()? {
                    Some(value) => value,
                    None => return Err(serde::de::Error::invalid_length(1, &self)),
                };
                Ok(MyTraitBox(de(t1)))
            }
        }
        deserializer.deserialize_tuple(2, Visitor)
    }
}

fn main() {
    let message = Message {
        s: vec![
            MyTraitBox::new(MyStruct {
                foo: String::from("a"),
                bar: 1,
            }),
            MyTraitBox::new(MyStruct {
                foo: String::from("b"),
                bar: 2,
            }),
            MyTraitBox::new(MyStruct {
                foo: String::from("c"),
                bar: 3,
            }),
            MyTraitBox::new(MyStruct {
                foo: String::from("d"),
                bar: 4,
            }),
        ],
    };

    let serialized = serde_json::to_string(&message).unwrap();
    println!("{}", serialized);
    std::fs::write("./resources/message.json", &serialized).unwrap();
    // let message = std::fs::read("./resources/message.json").unwrap();
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
    // MyStruct { foo: "abc", bar: 123 }
}
