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
    use web_sys::{Response, Request};
    use js_sys::Uint8Array;

    let url_str = url.to_string();
    if is_data_url(&url).unwrap() {
        Ok((url_str.clone(), url_str))
    } else if cache.contains_key(&url_str) {
        // url is in cache
        if !opt_silent {
            eprintln!("[ {} ] (from cache)", &url);
        }
        let data = cache.get(&url_str).unwrap();
        Ok((data.clone(), url_str))
    } else {
        // url not in cache, we request it
        let win = web_sys::window().unwrap();
        let promise = if opt_user_agent.is_empty() {
            win.fetch_with_str(&url_str)
        } else {
            let request = Request::new_with_str(&url_str)?;
            request.headers().set("User-Agent", opt_user_agent)?;
            win.fetch_with_request(&request)
        };

        let response = JsFuture::from(promise).await?;
        let response: Response = response.dyn_into()?;
        if !response.ok() {
            return Err(format!("Could not fetch {}: {}", &url_str, response.status_text()).into());
        }

        let res_url = response.url();
        if !opt_silent {
            if url_str == res_url {
                eprintln!("[ {} ]", &url_str);
            } else {
                eprintln!("[ {} -> {} ]", &url_str, &res_url);
            }
        }

        if as_dataurl {
            let buffer = JsFuture::from(response.array_buffer()?).await?;
            let data = Uint8Array::new(&buffer).to_vec();
            let mimetype = if mime.is_empty() {
                response.headers().get("Content-Type")?.unwrap_or_else(|| mime.to_string())
            } else {
                mime.to_string()
            };
            let dataurl = data_to_dataurl(&mimetype, &data);
            cache.insert(res_url.clone(), dataurl.clone());
            Ok((dataurl, res_url))
        } else {
            let content = JsFuture::from(response.text()?).await?.as_string().unwrap();
            cache.insert(res_url.clone(), content.clone());
            Ok((content, res_url))
        }
    }
}
