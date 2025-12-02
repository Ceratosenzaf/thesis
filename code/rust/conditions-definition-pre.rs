// Default implementation
impl<T> Foo<T> for MyType {
    fn foo(&self, x: T) {
        println!("Default Foo for MyType");
    }
}

// Specialized implementation for Display
#[when(T: Display)]
impl<T> Foo<T> for MyType {
    fn foo(&self, x: T) {
        println!("Foo for MyType where T implements Display");
    }
}

// Specialized implementation for &[_]
#[when(T = &[_])]
impl<T> Foo<T> for MyType {
    fn foo(&self, x: T) {
        println!("Foo for MyType where T is &[_]");
    }
}

// Specialized implementation for not Vec<_>
#[when(not(T = Vec<_>))]
impl<T> Foo<T> for MyType {
    fn foo(&self, x: T) {
        println!("Foo for MyType where T is not Vec<_>");
    }
}

// Specialized implementation for MyI8 or &u8
type MyI8 = i8;

#[when(any(T = MyI8, T = &u8))]
impl<T> Foo<T> for MyType {
    fn foo(&self, x: T) {
        println!("Foo for MyType where T is MyI8 or &u8");
    }
}
