use std::collections::HashMap;
use crate::utils::{data_to_dataurl, is_data_url};

#[cfg(not(target_arch = "wasm32"))]
pub fn retrieve_asset(
    cache: &mut HashMap<String, String>,
    url: &str,
    as_dataurl: bool,
    mime: &str,
    opt_user_agent: &str,
    opt_silent: bool,
    opt_insecure: bool,
) -> Result<(String, String), reqwest::Error> {
    use reqwest::header::{CONTENT_TYPE, USER_AGENT};
    use reqwest::Client;
    use std::time::Duration;

    if is_data_url(&url).unwrap() {
        Ok((url.to_string(), url.to_string()))
    } else {
        if cache.contains_key(&url.to_string()) {
            // url is in cache
            if !opt_silent {
                eprintln!("[ {} ] (from cache)", &url);
            }
            let data = cache.get(&url.to_string()).unwrap();
            Ok((data.to_string(), url.to_string()))
        } else {
            // url not in cache, we request it
            let client = Client::builder()
                .timeout(Duration::from_secs(10))
                .danger_accept_invalid_certs(opt_insecure)
                .build()?;
            let mut response = client.get(url).header(USER_AGENT, opt_user_agent).send()?;

            if !opt_silent {
                if url == response.url().as_str() {
                    eprintln!("[ {} ]", &url);
                } else {
                    eprintln!("[ {} -> {} ]", &url, &response.url().as_str());
                }
            }

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
                cache.insert(response.url().to_string(), dataurl.to_string());
                Ok((dataurl, response.url().to_string()))
            } else {
                let content = response.text().unwrap();
                // insert in cache
                cache.insert(response.url().to_string(), content.clone());
                Ok((content, response.url().to_string()))
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
    fn fetch_data(url: &str, user_agent: &str, wantBinary: bool) -> js_sys::Promise;
}

#[cfg(target_arch = "wasm32")]
pub async fn retrieve_asset(
    cache: &mut HashMap<String, String>,
    url: &str,
    as_dataurl: bool,
    mime: &str,
    opt_user_agent: &str,
    opt_silent: bool,
    _opt_insecure: bool,
) -> Result<(String, String), wasm_bindgen::JsValue> {
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen::JsCast;
    use web_sys::console;

    let url_str = url.to_string();
    if is_data_url(&url).unwrap() {
        Ok((url_str.clone(), url_str))
    } else if cache.contains_key(&url_str) {
        // url is in cache
        if !opt_silent {
            console::log_1(&JsValue::from(format!("Cache hit: {}", &url_str)));
        }
        let data = cache.get(&url_str).unwrap();
        Ok((data.clone(), url_str))
    } else {
        // url not in cache, we request it
        let fetched: FetchedData = JsFuture::from(fetch_data(&url_str, opt_user_agent, as_dataurl)).await?.dyn_into()?;

        let res_url = fetched.url();
        if !opt_silent {
            if url_str == res_url {
                console::log_1(&JsValue::from(format!("Retrieve: {}", &url_str)));
            } else {
                console::log_1(&JsValue::from(format!("Retrieve: {} -> {}", &url_str, &res_url)));
            }
        }

        if as_dataurl {
            let data = fetched.data();
            let mimetype = if mime.is_empty() {
                fetched.mime()
            } else {
                mime.to_string()
            };
            let dataurl = data_to_dataurl(&mimetype, &data);
            cache.insert(res_url.clone(), dataurl.clone());
            Ok((dataurl, res_url))
        } else {
            let content = fetched.text();
            cache.insert(res_url.clone(), content.clone());
            Ok((content, res_url))
        }
    }
}
