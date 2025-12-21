trait MyOtherTrait {}
impl<T: 'static> MyOtherTrait for T {}

trait MyTrait {
    fn my_method(&self);
}
impl<T> MyTrait for T {
    default fn my_method(&self) {
        println!("Generic implementation");
    }
}
impl<T: MyOtherTrait> MyTrait for T {
    fn my_method(&self) {
        println!("Specialized for 'static");
    }
}
fn main() {
    let x = "test"; // &str implements MyOtherTrait because it is 'static
    x.my_method();  // should call specialized impl but calls the generic one
}
