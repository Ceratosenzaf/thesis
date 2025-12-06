struct MyType;
trait Foo<T> {
    fn foo_default(&self, v: T);
    fn foo_i32(&self, v: i32);
    fn foo_u32(&self, v: u32);
}
impl<T> Foo<T> for MyType {
    fn foo_default(&self, v: T) {
        println!("default");
    }
    fn foo_i32(&self, v: i32) {
        println!("v is i32");
    }
    fn foo_u32(&self, v: u32) {
        println!("v is u32");
    }
}
