fn main() {
    let x = MyType;
    <MyType as Foo<Vec<i32>>>::foo(&x, vec![1, 2, 3]);
    <MyType as FooDisplay>::foo(&x, "hello".to_string());
    <MyType as FooSlice<i32>>::foo(&x, &[1, 2, 3]);
    <MyType as FooNotVec<&str>>::foo(&x, "hello");
    <MyType as FooMyI8>::foo(&x, 1i8);
    <MyType as FooRefU8>::foo(&x, &1u8);
}
