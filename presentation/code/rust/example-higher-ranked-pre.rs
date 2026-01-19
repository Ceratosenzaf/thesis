struct S;
trait Trait<T, U, V> {
    fn f(&self, t: T, u: U) -> V;
}
impl<T, U, V: Default> Trait<T, U, V> for S {
    fn f(&self, t: T, u: U) -> V {
        V::default()
    }
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
    let lambda = |s: &str, n: &i32| s.len() as u32 + *n as u32;
    let x: i32 = spec! { s.f("abc", 2); S; [&str, i32]; i32: Default }; // returns 0
    let y: u32 = spec! { s.f("abc", lambda); S;
      [&str, for<'a> fn(&str, &'a i32) -> u32]; u32: Default }; // returns 10
}
