struct S;
trait Trait<T, U, V> {
    fn f(&self, t: T, u: U) -> V;
}
trait Trait_fn<'b, V> {
    fn f(&self, t: &'b str, u: for<'a> fn(&'b str, &'a i32) -> V) -> V;
}
impl<'b, V> Trait_fn<'b, V> for S {
    fn f(&self, t: &'b str, u: for<'a> fn(&'b str, &'a i32) -> V) -> V {
        u(t, &7)
    }
}
fn main() {
    let s = S;
    let lambda = |a: &str, b: &i32| a.len() as i32 + *b;
    let res: i32 = <S as Trait_fn<_>>::f(&s, "abc", lambda); // returns 10
}
