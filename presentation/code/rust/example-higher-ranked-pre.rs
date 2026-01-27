struct S;
trait Trait<T, U, V> {
    fn f(&self, t: T, u: U) -> V;
}
#[when(all(
    T = &str, T: 'b, 
    U = for<'a> fn(T, &'a i32) -> V
))]
impl<'b, T, U, V> Trait<T, U, V> for S {
    fn f(&self, t: T, u: U) -> V { 
        u(t, &7)
    }
}
fn main() {
    let s = S;
    let lambda = |a: &str, b: &i32| a.len() as i32 + *b;
    let res: i32 = spec! { s.f("abc", lambda); S;
      [&str, for<'a> fn(&str, &'a i32) -> i32] }; // returns 10
}
