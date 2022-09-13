use std::fmt::Debug;
use std::ops::Range;

#[derive(Copy, Clone)]
struct MyRange {
    start: i32,
    end: i32,
}

struct MyRangeIter {
    cur: i32,
    end: i32,
}

impl Iterator for MyRangeIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            return None;
        }

        let res = self.cur;
        self.cur += 1;
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end as usize - self.cur as usize;
        (size, Some(size))
    }
}

impl IntoIterator for MyRange {
    type Item = i32;
    type IntoIter = MyRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        MyRangeIter {
            cur: self.start,
            end: self.end,
        }
    }
}

#[derive(Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

fn main() {
    let range = MyRange { start: 10, end: 20 };

    // `trait IntoIterator` is used.
    for x in range {
        println!("{:?}", x);
    }

    // Map to `Vector2`
    for x in range {
        let vec2 = Vector2 { x, y: x * 2 };
        println!("{:?}", vec2)
    }

    // 1. print all
    fn print_all<T: Debug>(items_to_print: &[T]) {
        for d in items_to_print {
            println!("{:?}", d);
        }
    }
    let mut all_vec2 = Vec::new();
    for x in range {
        let vec2 = Vector2 { x, y: x * 2 };
        all_vec2.push(vec2);
    }
    print_all(&all_vec2);
    // Use map and collect
    let all_vec2: Vec<_> = range.into_iter().map(|x| Vector2 { x, y: x * 2 }).collect();
    print_all(&all_vec2);

    // 2. Better print_all()
    fn better_print_all<D: Debug>(items_to_print: impl Iterator<Item = D>) {
        for x in items_to_print {
            println!("{:?}", x);
        }
    }
    // NOTE: `items` is not a `Vec`. It is a iterator.
    let items = range.into_iter().map(|x| Vector2 { x, y: x * 2 });
    better_print_all(items);

    // 3. Better and better
    fn better_better_print_all<D: Debug>(items_to_print: impl IntoIterator<Item = D>) {
        for x in items_to_print {
            println!("{:?}", x);
        }
    }
    // Now, we can print `Range` directly, it is no need to use `range.into_iter()`.
    better_better_print_all(Range { start: 1, end: 5 });
}
