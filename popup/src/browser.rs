use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_wasm_bindgen::from_value;
#[allow(deprecated)]
use wasm_bindgen::JsStatic;
use wasm_bindgen::JsValue;
use web_extensions_sys::Browser;

#[allow(deprecated)]
pub fn browser() -> &'static JsStatic<Browser> {
    web_extensions_sys::chrome()
    // TODO firefox
}

pub fn js_error(e: impl std::error::Error) -> JsValue {
    JsValue::from_str(&e.to_string())
}

pub async fn get_local<T: DeserializeOwned>(key: &str) -> Result<T, JsValue> {
    let obj = browser()
        .storage()
        .local()
        .get(&JsValue::from_str(key))
        .await?;
    let obj: Value = from_value(obj)?;

    if let Value::Object(mut obj) = obj {
        let content = obj.remove(key);
        if let Some(content) = content {
            let content = serde_json::from_value(content).map_err(js_error)?;
            return Ok(content);
        }
    }
    Err(JsValue::NULL)
}
