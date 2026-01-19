struct S;
trait Trait<T, U, V> {
    fn f(&self, t: T, u: U) -> V;
}
impl<T, U, V: Default> Trait<T, U, V> for S {
    fn f(&self, t: T, u: U) -> V {
        V::default()
    }
}
trait TraitFn<'b, V> {
    fn f(&self, t: &'b str, u: for<'a> fn(&'b str, &'a i32) -> V) -> V;
}
impl<'b, V> TraitFn<'b, V> for S {
    fn f(&self, t: &'b str, u: for<'a> fn(&'b str, &'a i32) -> V) -> V {
        u(t, &7)
    }
}
fn main() {
    let s = S;
    let lambda = |s: &str, n: &i32| s.len() as u32 + *n as u32;

    let x: i32 = <S as Trait<_, _, _>>::f(&s, "abc", 2);  // returns 0
    let y: u32 = <S as TraitFn<_>>::f(&s, "abc", lambda); // returns 10
}
