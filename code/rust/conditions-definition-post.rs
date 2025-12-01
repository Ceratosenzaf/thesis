struct MyType;

trait Foo<T> {
    fn foo(&self, x: T);
}

// Default implementation
impl<T> Foo<T> for ZST {
    fn foo(&self, x: T) {
        println!("Default Foo for MyType");
    }
}

// Specialized implementation for Display
trait FooDisplay {
    fn foo(&self, x: String);
}

impl<T: Display> FooDisplay<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo for MyType where T implements Display");
    }
}

// Specialized implementation for &[_]
trait FooSlice<T> {
    fn foo(&self, x: &[T]);
}

impl<T> FooSlice<T> for ZST {
    fn foo(&self, x: &[T]) {
        println!("Foo for MyType where T is &[_]");
    }
}

// Specialized implementation for not Vec<_>
trait FooNotVec<T> {
    fn foo(&self, x: T);
}

impl<T> FooNotVec<T> for ZST {
    fn foo(&self, x: T) {
        println!("Foo for MyType where T is not Vec<_>");
    }
}

// Specialized implementation for i8 or &u8
type MyI8 = i8;

trait FooMyI8 {
    fn foo(&self, x: MyI8);
}

impl FooMyI8 for ZST {
    fn foo(&self, x: MyI8) {
        println!("Foo for MyType where T is MyI8 or &u8");
    }
}

trait FooRefU8 {
    fn foo(&self, x: &u8);
}

impl FooRefU8 for ZST {
    fn foo(&self, x: &u8) {
        println!("Foo for MyType where T is MyI8 or &u8");
    }
}
