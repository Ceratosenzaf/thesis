struct MyType {}
struct MyTypeWithV<T> {
    v: T,
}
trait Foo {
    fn foo(&self);
}
impl Foo for MyTypeWithV<i32> {
    fn foo(&self) {
        println!("v is i32");
    }
}
impl Foo for MyTypeWithV<u32> {
    fn foo(&self) {
        println!("v is u32");
    }
}
