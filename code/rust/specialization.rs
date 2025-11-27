#![feature(specialization)]
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
        println!("Specialized implementation for i32");
    }
}
fn main() {
    10.my_method();     // prints "Specialized implementation for i32"
    10.5.my_method();   // prints "Generic implementation"
}