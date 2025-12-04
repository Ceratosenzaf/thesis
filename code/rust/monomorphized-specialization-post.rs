trait MyTrait {
    fn my_method(&self);
}
impl<T> MyTrait for T {
    fn my_method(&self) {
        println!("Generic implementation");
    }
}
trait MyTrait_i32 {
    fn my_method(&self);
}
impl MyTrait_i32 for i32 {
    fn my_method(&self) {
        println!("Specialized for i32");
    }
}
fn main() {
    <i32 as MyTrait_i32>::my_method(&10); // prints "Specialized for i32"
    <f64 as MyTrait>::my_method(&10.5);   // prints "Generic implementation"
}
