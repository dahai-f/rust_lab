struct StringContainer {
    s: String,
}

fn main() {
    let mut sc = StringContainer {
        s: "string value".into(),
    };
    let s_in_sc = return_ref(&mut sc);
    println!("{}", s_in_sc);
    let s_in_sc = return_ref(&mut sc);
    println!("{}", s_in_sc);
}

// fn return_ref() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn return_ref<'a, 'b: 'a>(sc: &'b mut StringContainer) -> &'a String {
    &sc.s
}
