mod utils;

use std::{collections::HashMap, ptr::eq};

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use json::object;

extern crate web_sys;
extern crate json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.]
    };

    JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen]
pub fn handle_json_from_js(val: &str) {
    set_panic_hook();

    let parsed = json::parse(val).unwrap();

    let instantiated = object!{
        "field1":{"0":"ex"},"field2":[[1,2],[3,4],[5,6]],"field3":[1,2,3,4]
    };

    // print the two objects
    web_sys::console::log_1(&parsed.to_string().into());
    web_sys::console::log_1(&"Hello".into());
    web_sys::console::log_1(&instantiated.to_string().into());


    let result =parsed.to_string().eq(&instantiated.to_string());
    web_sys::console::log_1(&result.into());
}

#[wasm_bindgen]
pub fn receive_example_from_js(val: &JsValue) {
    set_panic_hook();
    let example: Example = val.into_serde().unwrap();

    web_sys::console::log_1(&format!("{:?}", example).into());
}
