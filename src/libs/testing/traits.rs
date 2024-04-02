pub trait Testable {
    fn run(&self) -> ();
    fn should_panic(&self) -> bool;
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        super::serial::serial_print!("{}.........\t", core::any::type_name::<T>());

        if !self.should_panic() {
            self();
            super::serial::serial_println!("[ok]");
            return;
        }

        super::serial::serial_println!("[ok]");
    }

    fn should_panic(&self) -> bool {
        let func_name = core::any::type_name::<T>();
        func_name.ends_with("__should_panic")
    }
}
