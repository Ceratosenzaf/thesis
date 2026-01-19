trait MyTrait {
    fn my_method(&self);
}
impl<T> MyTrait for T {
    fn my_method(&self) {
        println!("Generic implementation");
    }
}
#[when(T = i32)]
impl<T> MyTrait for T {
    fn my_method(&self) {
        println!("Specialized for i32");
    }
}
fn main() {
    spec! { 10.my_method(); i32 };   // prints "Specialized for i32"
    spec! { 10.5.my_method(); f64 }; // prints "Generic implementation"
}
