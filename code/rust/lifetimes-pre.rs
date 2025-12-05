trait MyTrait {
    fn my_method(&self);
}
impl<T> MyTrait for T {
    fn my_method(&self) {
        println!("Generic implementation");
    }
}
#[when(T: 'static)] // or #[when(T = &'static _)]
impl<T> MyTrait for T {
    fn my_method(&self) {
        println!("Specialized for 'static");
    }
}
fn call_method(x: &'static i32, y: &i32) {
    spec! { x.my_method(); &'static i32 } // prints "Specialized for 'static"
    spec! { y.my_method(); &i32 }         // prints "Generic implementation"
}
