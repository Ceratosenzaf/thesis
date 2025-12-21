fn main() {
    let x = MyType;
    spec! { x.foo(vec![1, 2, 3]); MyType; [Vec<i32>] };
    spec! { x.foo("hello".to_string()); MyType; [String]; String: Display };
    spec! { x.foo(&[1, 2, 3]); MyType; [&[i32]] };
    spec! { x.foo("hello"); MyType; [&str] };
    spec! { x.foo(1i8); MyType; [i8]; MyI8 = i8 };
    spec! { x.foo(&1u8); MyType; [&u8] };
}
