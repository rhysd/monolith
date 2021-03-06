use crate::utils::{clean_url, data_to_dataurl, is_data_url};
#[cfg(target_arch = "wasm32")]
use crate::wasm_dummy_client::Client;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::blocking::Client;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
pub fn retrieve_asset(
    cache: &mut HashMap<String, String>,
    client: &Client,
    url: &str,
    as_dataurl: bool,
    mime: &str,
    opt_silent: bool,
) -> Result<(String, String), reqwest::Error> {
    let cache_key = clean_url(&url);

    if is_data_url(&url).unwrap() {
        Ok((url.to_string(), url.to_string()))
    } else {
        if cache.contains_key(&cache_key) {
            // url is in cache
            if !opt_silent {
                eprintln!("{} (from cache)", &url);
            }
            let data = cache.get(&cache_key).unwrap();
            Ok((data.to_string(), url.to_string()))
        } else {
            // url not in cache, we request it
            let mut response = client.get(url).send()?;
            let res_url = response.url().to_string();

            if !opt_silent {
                if url == res_url {
                    eprintln!("{}", &url);
                } else {
                    eprintln!("{} -> {}", &url, &res_url);
                }
            }

            let new_cache_key = clean_url(&res_url);

            if as_dataurl {
                // Convert response into a byte array
                let mut data: Vec<u8> = vec![];
                response.copy_to(&mut data)?;

                // Attempt to obtain MIME type by reading the Content-Type header
                let mimetype = if mime == "" {
                    response
                        .headers()
                        .get(CONTENT_TYPE)
                        .and_then(|header| header.to_str().ok())
                        .unwrap_or(&mime)
                } else {
                    mime
                };
                let dataurl = data_to_dataurl(&mimetype, &data);
                // insert in cache
                cache.insert(new_cache_key, dataurl.clone());
                Ok((dataurl, res_url))
            } else {
                let content = response.text().unwrap();
                // insert in cache
                cache.insert(new_cache_key, content.clone());
                Ok((content, res_url))
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/src/fetch.js")]
extern "C" {
    type FetchedData;
    #[wasm_bindgen(method, getter)]
    fn data(this: &FetchedData) -> Vec<u8>;
    #[wasm_bindgen(method, getter)]
    fn text(this: &FetchedData) -> String;
    #[wasm_bindgen(method, getter)]
    fn mime(this: &FetchedData) -> String;
    #[wasm_bindgen(method, getter)]
    fn url(this: &FetchedData) -> String;
    #[wasm_bindgen(js_name = fetchData)]
    fn fetch_data(url: &str, wantBinary: bool) -> js_sys::Promise;
}

#[cfg(target_arch = "wasm32")]
pub async fn retrieve_asset(
    cache: &mut HashMap<String, String>,
    _client: &Client,
    url: &str,
    as_dataurl: bool,
    mime: &str,
    opt_silent: bool,
) -> Result<(String, String), wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::console;

    let url_str = url.to_string();
    if url_str.is_empty() {
        Ok(("".to_string(), "".to_string()))
    } else if is_data_url(&url).unwrap() {
        Ok((url_str.clone(), url_str))
    } else {
        let cache_key = clean_url(&url);
        if cache.contains_key(&cache_key) {
            // url is in cache
            if !opt_silent {
                console::log_1(&JsValue::from(format!("Cache hit: {}", &url_str)));
            }
            let data = cache.get(&cache_key).unwrap();
            Ok((data.clone(), url_str))
        } else {
            // url not in cache, we request it
            let fetched: FetchedData = JsFuture::from(fetch_data(&url_str, as_dataurl))
                .await?
                .dyn_into()?;

            let res_url = fetched.url();
            if !opt_silent {
                if url_str == res_url {
                    console::log_1(&JsValue::from(format!("Retrieve: {}", &url_str)));
                } else {
                    console::log_1(&JsValue::from(format!(
                        "Retrieve: {} -> {}",
                        &url_str, &res_url
                    )));
                }
            }

            let new_cache_key = clean_url(&res_url);

            if as_dataurl {
                let data = fetched.data();
                let mimetype = if mime.is_empty() {
                    fetched.mime()
                } else {
                    mime.to_string()
                };
                let dataurl = data_to_dataurl(&mimetype, &data);
                cache.insert(new_cache_key, dataurl.clone());
                Ok((dataurl, res_url))
            } else {
                let content = fetched.text();
                cache.insert(new_cache_key, content.clone());
                Ok((content, res_url))
            }
        }
    }
}
