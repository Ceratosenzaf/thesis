trait Bar<T, U, V> { fn bar(&self, t: T, u: U) -> V; }
#[when(all(
    T = &str, T: 'b, 
    U = for<'a> fn(T, &'a i32) -> V
))]
impl<'b, T, U, V> Bar<T, U, V> for MyType {
    fn bar(&self, t: T, u: U) -> V { 
        u(t, &7)
    }
}
fn main() {
    let x = MyType;
    let lambda = |a: &str, b: &i32| a.len() as u32 + *b as u32;
    let res: u32 = spec! { x.bar("abc", lambda); MyType;
      [&str, for<'a> fn(&str, &'a i32) -> u32] }; // returns 10
}
