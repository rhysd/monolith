#[macro_use]
extern crate clap;

mod args;
mod macros;

use crate::args::AppArgs;
use monolith::html::{html_to_dom, stringify_document, walk_and_embed_assets};
use monolith::http::retrieve_asset;
use monolith::utils::is_valid_url;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Url;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Error, Write};
use std::process;
use std::time::Duration;

enum Output {
    Stdout(io::Stdout),
    File(File),
}

impl Output {
    fn new(file_path: &str) -> Result<Output, Error> {
        if file_path.is_empty() {
            Ok(Output::Stdout(io::stdout()))
        } else {
            Ok(Output::File(File::create(file_path)?))
        }
    }

    fn writeln_str(&mut self, s: &str) -> Result<(), Error> {
        match self {
            Output::Stdout(stdout) => {
                writeln!(stdout, "{}", s)?;
                stdout.flush()
            }
            Output::File(f) => {
                writeln!(f, "{}", s)?;
                f.flush()
            }
        }
    }
}

fn main() {
    let app_args = AppArgs::get();

    if !is_valid_url(app_args.url_target.as_str()) {
        eprintln!(
            "Only HTTP and HTTPS URLs are allowed but got: {}",
            &app_args.url_target
        );
        process::exit(1);
    }

    let mut output = Output::new(&app_args.output).expect("Could not prepare output");

    // Initialize client
    let mut cache = HashMap::new();
    let mut header_map = HeaderMap::new();
    header_map.insert(
        USER_AGENT,
        HeaderValue::from_str(&app_args.user_agent).expect("Invalid User-Agent header specified"),
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(app_args.insecure)
        .default_headers(header_map)
        .build()
        .expect("Failed to initialize HTTP client");

    // Retrieve root document
    let (data, final_url) = retrieve_asset(
        &mut cache,
        &client,
        app_args.url_target.as_str(),
        false,
        "",
        app_args.silent,
    )
    .expect("Could not retrieve assets in HTML");
    let downloaded_time = time::now_utc();
    let dom = html_to_dom(&data);

    walk_and_embed_assets(
        &mut cache,
        &client,
        &final_url,
        &dom.document,
        app_args.no_css,
        app_args.no_js,
        app_args.no_images,
        app_args.silent,
        app_args.no_frames,
    );

    let mut html: String = stringify_document(
        &dom.document,
        app_args.no_css,
        app_args.no_frames,
        app_args.no_js,
        app_args.no_images,
        app_args.isolate,
    );

    if !app_args.no_context {
        // Safe to unwrap: We just put this through an HTTP request
        let mut clean_url = Url::parse(&final_url).unwrap();
        clean_url.set_fragment(None);
        // Safe to unwrap: must have a protocol and thus base 'cause we just used it.
        clean_url.set_password(None).unwrap();
        clean_url.set_username("").unwrap();
        html.insert_str(
            0,
            &format!(
                "<!--- Downloaded from {} on {} using {} v{} -->\n",
                &clean_url,
                downloaded_time.rfc3339(),
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ),
        );
    }

    output
        .writeln_str(&html)
        .expect("Could not write HTML output");
}
