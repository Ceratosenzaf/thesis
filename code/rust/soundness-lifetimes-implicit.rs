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
