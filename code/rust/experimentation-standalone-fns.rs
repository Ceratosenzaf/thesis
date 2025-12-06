struct MyType;
fn foo_default<T>(x: &MyType, v: T) {
    println!("default");
}
fn foo_i32(x: &MyType, v: i32) {
    println!("v is i32");
}
fn foo_u32(x: &MyType, v: u32) {
    println!("v is u32");
}
