#[derive(Debug, Copy, Clone)]
struct Integer32 {
    i: i32,
}
#[derive(Debug, Copy, Clone)]
struct Float32 {
    f: f32,
}

trait Scalar {
    const ZERO: Self;
}
impl Scalar for Integer32 {
    const ZERO: Self = Integer32 { i: 0 };
}
impl Scalar for Float32 {
    const ZERO: Self = Float32 { f: 0.0 };
}

trait Add<Rhs> {
    type Result;
    fn add(self, rhs: Rhs) -> Self::Result;
}
impl Add<Integer32> for Integer32 {
    type Result = Integer32;

    fn add(self, rhs: Integer32) -> Self::Result {
        Integer32 { i: self.i + rhs.i }
    }
}
impl Add<Float32> for Float32 {
    type Result = Float32;

    fn add(self, rhs: Float32) -> Self::Result {
        Self::Result { f: self.f + rhs.f }
    }
}

trait AddGenericResult<Rhs, Result> {
    fn add_generic_result(self, rhs: Rhs) -> Result;
}
impl AddGenericResult<Integer32, Integer32> for Integer32 {
    fn add_generic_result(self, rhs: Integer32) -> Integer32 {
        Integer32 { i: self.i + rhs.i }
    }
}
impl AddGenericResult<Integer32, Float32> for Integer32 {
    fn add_generic_result(self, rhs: Integer32) -> Float32 {
        Float32 {
            f: (self.i + rhs.i) as f32,
        }
    }
}

trait MyFrom<Other> {
    fn my_from(other: Other) -> Self;
}
trait MyInto<Result> {
    fn my_into(self) -> Result;
}
impl MyFrom<Integer32> for Float32 {
    fn my_from(other: Integer32) -> Self {
        Self { f: other.i as f32 }
    }
}
impl<F, T> MyInto<T> for F
where
    T: MyFrom<F>,
{
    fn my_into(self) -> T {
        T::my_from(self)
    }
}

fn add<A, B, R>(a: A, b: B) -> R
where
    A: Add<B, Result = R>,
{
    a.add(b)
}

fn main() {
    let a = Integer32 { i: 10 };
    let a_copy = a;
    println!("a: {:?}, a_copy: {:?}", a, a_copy);

    let b = Float32::ZERO;
    // ERROR, because we did not implement Add<Float32> for Integer.
    // let c = add(a, b);

    let a = Float32::my_from(a);
    let c = add(a, b);
    println!("c: {:?}", c);

    let a = Integer32::ZERO;
    let b = Integer32 { i: 20 };
    let cf: Float32 = a.add_generic_result(b);
    println!("cf: {:?}", cf);
    let ci: Integer32 = a.add_generic_result(b);
    println!("ci: {:?}", ci);
}
