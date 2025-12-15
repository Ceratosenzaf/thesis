struct MyType;
fn foo_default<T>(x: &MyType, v: T) {
    println!("default");
}
fn foo_i32(x: &MyType, v: i32) {
    println!("v is i32");
}
fn call<T>(my_type: &MyType, v: T) {
    foo_default(my_type, v);
}
