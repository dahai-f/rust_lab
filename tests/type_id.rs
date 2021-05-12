use std::any::TypeId;

#[test]
fn type_id() {
    let id_i32 = TypeId::of::<i32>();
    let id_u32 = TypeId::of::<u32>();
    let id_i32_u32 = TypeId::of::<(i32, u32)>();

    println!("{:?}", id_i32);
    println!("{:?}", id_u32);
    println!("{:?}", id_i32_u32);

    assert_ne!(id_i32, id_u32);
    assert_ne!(id_i32, id_i32_u32);
    assert_ne!(id_u32, id_i32_u32);

    assert_eq!(id_i32, TypeId::of::<i32>());
    assert_eq!(id_u32, TypeId::of::<u32>());
    assert_eq!(id_i32_u32, TypeId::of::<(i32, u32)>());
}
