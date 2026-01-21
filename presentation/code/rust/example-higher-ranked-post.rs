struct S;
trait Trait<T, U, V> {
    fn f(&self, t: T, u: U) -> V;
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
    let res: u32 = <S as TraitFn<_>>::f(&s, "abc", lambda); // returns 10
}
