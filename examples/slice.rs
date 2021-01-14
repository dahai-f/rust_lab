fn main() {
    let hello_world = "hello, world.";
    let hello = &hello_world[..5];
    let world = &hello_world[7..];
    println!("{}: len: {}", hello, hello.len());
    println!("{}: len: {}", world, world.len());

    let mut nums = vec![0, 1, 2, 3, 4];
    let nums_a = &mut nums[0];
    // let nums_b = &mut nums[1];
    *nums_a = 10;
}
