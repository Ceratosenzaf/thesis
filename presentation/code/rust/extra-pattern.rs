struct S;
impl S {
    fn my_method_default<T>(&self, x: T) {
        println!("Generic implementation");
    }
    fn my_method_i32(&self, x: i32) {
        println!("Specialized for i32");
    }
}
fn main() {
    let s = S;
    s.my_method_i32(10);         // prints "Specialized for i32"
    s.my_method_default(10.5);   // prints "Generic implementation"
}