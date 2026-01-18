trait MyTrait {
    fn my_method(&self);
}
impl<T> MyTrait for T {
    fn my_method(&self) {
        println!("Generic implementation");
    }
}
impl MyTrait for i32 {
    fn my_method(&self) {
        println!("Specialized for i32");
    }
}
