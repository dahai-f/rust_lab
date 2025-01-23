use rayon::prelude::*;

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    vec1.par_iter()
        .zip(vec2.par_iter())
        .flat_map(|(a, b)| vec![a, b])
        .filter_map(|&v| if v >= 2 { Some(v * 2) } else { None })
        .for_each(|v| println!("{v}"));

    let s = "hello";
    let ss: String = s.par_chars().filter_map(|c| Some(c)).collect();
    println!("{ss}");
}
