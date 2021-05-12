use std::slice::SliceIndex;

/// 获取切片中的某个或某些元素
///
/// # Params
///
/// * `index` - 索引。可以是单个元素的索引，也可以是Range
///
/// # Examples
///
/// ```
/// use rust_lab::index;
///
/// let s = [10, 11, 12, 13, 14, 15];
/// assert_eq!(index(&s, 2), &12);
/// assert_eq!(index(&s, 2..5), &[12, 13, 14]);
/// ```
///
pub fn index<T, I: SliceIndex<[T]>>(a: &[T], index: I) -> &I::Output {
    &a[index]
}

pub fn index_mut<T, I: SliceIndex<[T]>>(a: &mut [T], index: I) -> &mut I::Output {
    &mut a[index]
}

pub mod thread_pool;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut s = [10, 11, 12, 13, 14, 15];
        let two = index_mut(&mut s, 2);
        // let two = &mut s[2];
        assert_eq!(two, &mut 12);
        let range = index_mut(&mut s, 3..5);
        // let range = &mut s[3..5];
        assert_eq!(range, &mut [13, 14]);
    }
}
