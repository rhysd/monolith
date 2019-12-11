use std::result::Result;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use crate::html::{html_to_dom, stringify_document, walk_and_embed_assets};
use crate::http::retrieve_asset;
use crate::utils::is_valid_url;

// Entrypoints for WebAssembly port of monolith. This function will be called from JavaScript

#[wasm_bindgen(js_name = monolithOfUrl)]
pub async fn monolith_of_url(
    url_target: String,
    no_css: bool,
    no_frames: bool,
    no_images: bool,
    no_js: bool,
    insecure: bool,
    isolate: bool,
    silent: bool,
    user_agent: String,
) -> Result<String, JsValue> {
    if !is_valid_url(url_target.as_str()) {
        return Err(format!("Not a valid URL: {}", url_target).into());
    }

    let cache = &mut HashMap::new();
    let (data, final_url) = retrieve_asset(
        cache,
        url_target.as_str(),
        false,
        "",
        user_agent.as_str(),
        silent,
        insecure,
    )
    .await?;
    let dom = html_to_dom(&data);

    walk_and_embed_assets(
        cache,
        &final_url,
        &dom.document,
        no_css,
        no_js,
        no_images,
        user_agent.as_str(),
        silent,
        insecure,
        no_frames,
    ).await?;

    let html = stringify_document(
        &dom.document,
        no_css,
        no_frames,
        no_js,
        no_images,
        isolate,
    );

    Ok(html)
}

#[wasm_bindgen(js_name = monolithOfHtml)]
pub async fn monolith_of_html(
    html: String,
    final_url: String,
    no_css: bool,
    no_frames: bool,
    no_images: bool,
    no_js: bool,
    insecure: bool,
    isolate: bool,
    silent: bool,
    user_agent: String,
) -> Result<String, JsValue> {
    let cache = &mut HashMap::new();
    let dom = html_to_dom(&html);

    walk_and_embed_assets(
        cache,
        &final_url,
        &dom.document,
        no_css,
        no_js,
        no_images,
        user_agent.as_str(),
        silent,
        insecure,
        no_frames,
    ).await?;

    let html = stringify_document(
        &dom.document,
        no_css,
        no_frames,
        no_js,
        no_images,
        isolate,
    );

    Ok(html)
}
