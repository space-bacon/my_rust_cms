use std::env;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use reqwasm::http::{Request};
use web_sys::console;
use yew::Callback;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CMSComponent {
    pub id: Option<i32>,
    pub name: String,
    pub html: String,
    pub css: String,
    pub js: String,
}

fn get_api_base_url() -> String {
    env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

pub async fn save_component(component: CMSComponent) -> Result<(), JsValue> {
    let url = format!("{}/api/components", get_api_base_url());

    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&component).unwrap())
        .send()
        .await;

    match response {
        Ok(res) if res.ok() => {
            console::log_1(&"Component saved successfully!".into());
            Ok(())
        }
        Ok(res) => {
            let error_message = format!("Failed to save component: {:?}", res);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
        Err(err) => {
            let error_message = format!("Request error: {:?}", err);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
    }
}

pub fn fetch_components(on_success: Callback<Vec<CMSComponent>>, on_failure: Callback<JsValue>) {
    let url = format!("{}/api/components", get_api_base_url());
    
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::get(&url).send().await;
        
        match response {
            Ok(res) if res.ok() => {
                let json = res.json::<Vec<CMSComponent>>().await;
                match json {
                    Ok(components) => on_success.emit(components),
                    Err(err) => on_failure.emit(JsValue::from_str(&format!("Failed to parse components: {:?}", err))),
                }
            }
            Ok(res) => on_failure.emit(JsValue::from_str(&format!("Failed to fetch components: {:?}", res))),
            Err(err) => on_failure.emit(JsValue::from_str(&format!("Request error: {:?}", err))),
        }
    });
}

pub async fn delete_component(id: i32) -> Result<(), JsValue> {
    let url = format!("{}/api/components/{}", get_api_base_url(), id);

    let response = Request::delete(&url).send().await;

    match response {
        Ok(res) if res.ok() => {
            console::log_1(&"Component deleted successfully!".into());
            Ok(())
        }
        Ok(res) => {
            let error_message = format!("Failed to delete component: {:?}", res);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
        Err(err) => {
            let error_message = format!("Request error: {:?}", err);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
    }
}
