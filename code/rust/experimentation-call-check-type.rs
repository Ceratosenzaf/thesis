fn call<T: 'static>(my_type: &MyType, v: T) {
    match std::any::TypeId::of::<T>() {
        id if id == std::any::TypeId::of::<i32>() => {
            // SAFETY: We just checked that T is i32
            let v_i32 = unsafe { std::mem::transmute_copy::<T, i32>(&v) };
            // TODO: Call the specialized version
        }
        _ => {
            // TODO: Call the default version
        }
    }
}
