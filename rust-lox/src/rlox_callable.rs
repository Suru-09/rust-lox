pub mod rlox_callable {
    use std::any::Any;
    pub trait RLoxCallable {
        fn arity(&self) -> usize;
        fn call(&self, args: Vec<Box<dyn Any>>) -> Box<dyn Any>;
    }

    pub struct Clock {}

    impl RLoxCallable for Clock {
        fn arity(&self) -> usize {
            0
        }

        fn call(&self, _args: Vec<Box<dyn Any>>) -> Box<dyn Any> {
            Box::new(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| panic!("Could not get time since epoch"))
                .as_secs_f64())
        }
    }
}

