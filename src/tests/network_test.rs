#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;

    wasm_bindgen_test_configure!(run_in_browser);

    // Example function to fetch data (this would be part of your application code)
    async fn fetch_data(url: &str) -> Result<String, String> {
        let window = web_sys::window().ok_or("No global `window` exists")?;
        let resp_value = JsFuture::from(window.fetch_with_str(url)).await.map_err(|_| "Failed to fetch")?;
        let resp: Response = resp_value.dyn_into().unwrap();

        if resp.ok() {
            let text = JsFuture::from(resp.text().unwrap()).await.map_err(|_| "Failed to read response text")?;
            Ok(text.as_string().unwrap_or_else(|| "Empty response".to_string()))
        } else {
            Err("Non-200 response".to_string())
        }
    }

    #[wasm_bindgen_test]
    async fn test_fetch_data() {
        let data = fetch_data("https://google.com").await;
        assert!(data.is_ok());
    }
}
