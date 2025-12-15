struct MyType {}
struct MyTypeWithV<'a, T> {
    my_type: &'a MyType,
    v: T,
}
trait Foo {
    fn foo(&self);
}
impl<'a> Foo for MyTypeWithV<'a, i32> {
    fn foo(&self) {
        println!("v is i32");
    }
}
impl<'a> Foo for MyTypeWithV<'a, u32> {
    fn foo(&self) {
        println!("v is u32");
    }
}
fn call<'a, T>(my_type: &'a MyType, v: T)
where
    MyTypeWithV<'a, T>: Foo,
{
    MyTypeWithV { my_type, v }.foo();
}
