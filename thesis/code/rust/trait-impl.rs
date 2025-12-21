trait MyTrait { fn do_something(&self) }

impl MyTrait for i32 {
    fn do_something(&self) {}
}
