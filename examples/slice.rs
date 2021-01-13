fn main() {
    let string = "hello, world.";
    let slice = &string[7..];
    println!("{}", slice.len());
    println!("{}", slice);
}
