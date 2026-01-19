error[E0119]: conflicting implementations of trait `MyTrait` for type `i32`
 --> src/main.rs:7:1
  |
2 | impl<T> MyTrait for T {
  | --------------------- first implementation here
...
7 | impl MyTrait for i32 {
  | ^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `i32`