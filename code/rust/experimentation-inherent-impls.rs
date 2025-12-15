struct MyType;
impl MyType {
    fn foo_default<T>(&self, v: T) {
        println!("default");
    }
    fn foo_i32(&self, v: i32) {
        println!("v is i32");
    }
}
fn call<T>(my_type: &MyType, v: T) {
    my_type.foo_default(v);
}
