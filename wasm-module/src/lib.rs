use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
mod lang_analyzer;

#[derive(Serialize, Deserialize)]
pub struct Values {
    pub lang: String,
    pub value: usize
}

#[wasm_bindgen]
pub async fn draw(values: JsValue) -> Result<JsValue, String> {
    let values = match lang_analyzer::get_values(values) {
        Ok(values) => values,
        Err(err) => {
            return Err(err);
        }
    };

    if values.len() == 0 {
        return Err("hash map is empty".to_string());
    }

    let window = match web_sys::window() {
        Some(window) => window,
        _ => {
            return Err("no window object".to_string());
        }
    };

    let document = match window.document() {
        Some(window) => window,
        _ => {
            return Err("no document object".to_string());
        }
    };
     
    match lang_analyzer::draw_pie_diagram(&document, &values) {
        Ok(()) => (),
        Err(err) => {
            return Err(err.to_string());
        }
    };
    
    match lang_analyzer::draw_bar_diagram(&document, &values) {
        Ok(()) => (),
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let lang_colors = lang_analyzer::get_colors_map(values.clone());

    let response = (&values, &lang_colors);

    match serde_wasm_bindgen::to_value(&response) {
        Ok(response) => Ok(response),
        Err(_) => Err("cannot convert values".to_string())
    }
}