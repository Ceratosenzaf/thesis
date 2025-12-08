struct MyType;
trait Foo<T> {
    fn foo(&self, v: T);
}
impl<T> Foo<T> for MyType {
    fn foo(&self, v: T) {
        println!("default");
    }
}
#[when(T = i32)]
impl<T> Foo<T> for MyType {
    fn foo(&self, v: T) {
        println!("v is i32");
    }
}
