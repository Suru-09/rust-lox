pub mod rlox_callable {
    use std::any::Any;
    pub trait RLoxCallable {
        fn arity(&self) -> usize;
        fn call(&self, args: Vec<Box<dyn Any>>) -> Box<dyn Any>;
    }
}

