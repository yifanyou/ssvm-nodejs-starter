use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}

// new func
#[wasm_bindgen]
pub fn says(s1: &str, s2: $str) -> String {
  println!("The Rust function say() received {} {}", s1, s2);
  let r = String::from("hello ");
  return r + s1 + s2;
}