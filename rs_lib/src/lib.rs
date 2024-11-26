use wasm_bindgen::prelude::*;
// use std::fs;

/*
#[wasm_bindgen]
pub fn poc(file_path: String) -> String{
    let contents = fs::read_to_string(file_path).expect("The file should be read");
    return contents;
}
*/

#[link(name = "pocrust", kind = "static")]
extern "C" {
    // fn pocc_f() -> *const c_char;
    pub fn pocc_f() -> f32;
}

/*
#[wasm_bindgen]
pub fn rust_string() -> String {
  let cstr = unsafe { CStr::from_ptr(pocc_f()) };
  String::from_utf8_lossy(cstr.to_bytes()).to_string()
}
*/

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
pub struct Greeter {
  name: String,
}

#[wasm_bindgen]
impl Greeter {
  #[wasm_bindgen(constructor)]
  pub fn new(name: String) -> Self {
    Self { name }
  }

  pub fn greet(&self) -> String {
    format!("Hello {}!", self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }

  #[test]
  fn it_greets() {
    let greeter = Greeter::new("world".into());
    assert_eq!(greeter.greet(), "Hello world!");
  }
}
