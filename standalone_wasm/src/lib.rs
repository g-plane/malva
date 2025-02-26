use wasm_bindgen::JsValue;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::wasm_bindgen)]
pub fn format(input: String, syntax: JsValue, config: JsValue) -> Result<String, JsValue> {
    let syntax = serde_wasm_bindgen::from_value(syntax)?;
    let options = serde_wasm_bindgen::from_value(config)?;
    match malva::format_text(&input, syntax, &options) {
        Ok(output) => Ok(output),
        Err(error) => Err(serde_wasm_bindgen::to_value(&error.to_string())?),
    }
}
