fn main() {
    let x = MyType;
    spec! { x.foo(vec![1, 2, 3]); Vec<i32> };
    spec! { x.foo("hello".to_string()); String; String: Display };
    spec! { x.foo(&[1, 2, 3]); &[i32] };
    spec! { x.foo("hello"); &str };
    spec! { x.foo(1i8); i8; MyI8 = i8 };
    spec! { x.foo(&1u8); &u8 };
}
