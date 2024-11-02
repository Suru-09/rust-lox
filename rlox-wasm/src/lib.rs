use rlox::{execute_file, init};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ResultMessage {
    output: String,
    errors: String,
}

#[wasm_bindgen]
impl ResultMessage {
    #[wasm_bindgen(constructor)]
    pub fn new(output: String, errors: String) -> ResultMessage {
        Self { output, errors }
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn get_errors(&self) -> String {
        self.errors.clone()
    }

    pub fn set_output(&mut self, val: String) {
        self.output = val;
    }

    pub fn set_errors(&mut self, val: String) {
        self.errors = val;
    }
}

#[wasm_bindgen]
pub fn init_interpreter() {
    init();
}

#[wasm_bindgen]
pub fn run_file(str: String) -> ResultMessage {
    let (output, errors) = execute_file(str);
    ResultMessage { output, errors }
}
