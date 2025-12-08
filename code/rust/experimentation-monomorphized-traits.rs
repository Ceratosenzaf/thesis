struct MyType;
trait FooDefault<T> {
    fn foo(&self, v: T);
}
trait FooI32 {
    fn foo(&self, v: i32);
}
impl FooDefault for MyType {
    fn foo(&self, v: T) {
        println!("default");
    }
}
impl FooI32 for MyType {
    fn foo(&self, v: i32) {
        println!("v is i32");
    }
}
