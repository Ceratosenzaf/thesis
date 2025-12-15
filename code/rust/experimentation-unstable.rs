#![feature(specialization)]
struct MyType;
trait Foo<T> {
    fn foo(&self, v: T);
}
impl<T> Foo<T> for MyType {
    default fn foo(&self, v: T) {
        println!("default");
    }
}
impl Foo<i32> for MyType {
    fn foo(&self, v: i32) {
        println!("v is i32");
    }
}
fn call<T>(my_type: &MyType, v: T) {
    my_type.foo(v);
}
