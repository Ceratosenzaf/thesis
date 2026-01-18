error[E0119]: conflicting implementations of trait `MyTrait` for type `i32`
 --> src/main.rs:9:1
  |
4 | impl<T> MyTrait for T {
  | --------------------- first implementation here
...
9 | impl MyTrait for i32 {
  | ^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `i32`