use rayon::prelude::*;

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    vec1.par_iter()
        .zip(vec2.par_iter())
        .flat_map(|(a, b)| vec![a, b])
        .for_each(|v| println!("{v}"));
}
