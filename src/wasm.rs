use crate::html::{html_to_dom, stringify_document, walk_and_embed_assets};
use crate::http::retrieve_asset;
use crate::utils::is_valid_url;
use std::collections::HashMap;
use std::result::Result;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MonolithOptions {
    no_css: bool,
    no_frames: bool,
    no_images: bool,
    no_js: bool,
    isolate: bool,
    silent: bool,
}

#[wasm_bindgen]
impl MonolithOptions {
    pub fn new() -> Self {
        Self {
            no_css: false,
            no_frames: false,
            no_images: false,
            no_js: false,
            isolate: false,
            silent: false,
        }
    }

    #[wasm_bindgen(js_name = noCss)]
    pub fn no_css(&mut self, b: bool) {
        self.no_css = b;
    }

    #[wasm_bindgen(js_name = noFrames)]
    pub fn no_frames(&mut self, b: bool) {
        self.no_frames = b;
    }

    #[wasm_bindgen(js_name = noImages)]
    pub fn no_images(&mut self, b: bool) {
        self.no_images = b;
    }

    #[wasm_bindgen(js_name = noJs)]
    pub fn no_js(&mut self, b: bool) {
        self.no_js = b;
    }

    pub fn isolate(&mut self, b: bool) {
        self.isolate = b;
    }

    pub fn silent(&mut self, b: bool) {
        self.silent = b;
    }
}

// Entrypoints for WebAssembly port of monolith. This function will be called from JavaScript

#[wasm_bindgen(js_name = monolithOfUrl)]
pub async fn monolith_of_url(
    url_target: String,
    options: MonolithOptions,
) -> Result<String, JsValue> {
    if !is_valid_url(url_target.as_str()) {
        return Err(format!("Not a valid URL: {}", url_target).into());
    }

    let MonolithOptions {
        no_css,
        no_frames,
        no_images,
        no_js,
        isolate,
        silent,
    } = options;

    let cache = &mut HashMap::new();
    let (data, final_url) = retrieve_asset(cache, url_target.as_str(), false, "", silent).await?;
    let dom = html_to_dom(&data);

    walk_and_embed_assets(
        cache,
        &final_url,
        &dom.document,
        no_css,
        no_js,
        no_images,
        silent,
        no_frames,
    )
    .await;

    let html = stringify_document(&dom.document, no_css, no_frames, no_js, no_images, isolate);

    Ok(html)
}

#[wasm_bindgen(js_name = monolithOfHtml)]
pub async fn monolith_of_html(
    html: String,
    final_url: String,
    options: MonolithOptions,
) -> Result<String, JsValue> {
    let MonolithOptions {
        no_css,
        no_frames,
        no_images,
        no_js,
        isolate,
        silent,
    } = options;
    let cache = &mut HashMap::new();
    let dom = html_to_dom(&html);

    walk_and_embed_assets(
        cache,
        &final_url,
        &dom.document,
        no_css,
        no_js,
        no_images,
        silent,
        no_frames,
    )
    .await;

    let html = stringify_document(&dom.document, no_css, no_frames, no_js, no_images, isolate);

    Ok(html)
}
