trait Bar<T, U, V> { fn bar(&self, t: T, u: U) -> V; }
trait BarFn<'b, V> {
    fn bar(&self, t: &'b str, u: for<'a> fn(&'b str, &'a i32) -> V) -> V;
}
impl<'b, V> BarFn<'b, V> for MyType {
    fn bar(&self, t: &'b str, u: for<'a> fn(&'b str, &'a i32) -> V) -> V {
        u(t, &7)
    }
}
fn main() {
    let x = MyType;
    let lambda = |a: &str, b: &i32| a.len() as u32 + *b as u32;
    let res: u32 = <MyType as BarFn<_>>::bar(&x, "abc", lambda); // returns 10
}
