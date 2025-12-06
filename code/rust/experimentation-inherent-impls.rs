struct MyType;
impl MyType {
    fn foo_default<T>(&self, v: T) {
        println!("default");
    }
    fn foo_i32(&self, v: i32) {
        println!("v is i32");
    }
    fn foo_u32(&self, v: u32) {
        println!("v is u32");
    }
}
