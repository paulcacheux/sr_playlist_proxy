//! Playlist proxy between a media player and a remote playlist/media server

#![warn(missing_docs)]

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use colored::Colorize;
use reqwest::Url;
use std::sync::atomic::{AtomicBool, Ordering};

mod type_guesser;
mod url_rewriting;
use type_guesser::FileType;

/// Fetch and returns the body of the remote content at `url`
async fn fetch_url(url: Url) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;
    Ok(body.into_iter().collect())
}

/// Returns the final target URL, the one linking to the actual media
fn build_target_url(base_url: &Url, path: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let mut path_iter = path.split("/").peekable();

    // skip the first segment if it is empty
    if let Some(&"") = path_iter.peek() {
        path_iter.next();
    }

    path_iter.peek();
    match path_iter.next() {
        Some(seg) if seg == url_rewriting::URL_REWRITING_PREFIX => {
            if let Some(b64_url) = path_iter.next() {
                let url_bytes = base64::decode_config(b64_url, base64::URL_SAFE)?;
                let url_str = String::from_utf8(url_bytes)?;
                let url = Url::parse(&url_str)?;
                return Ok(url);
            }
        }
        _ => {}
    }

    let url = base_url.join(path)?;
    Ok(url)
}

/// Route used to proxy request to `base_url`
///
/// # Arguments
/// Arguments are passed via actix data system
/// * `base_url` - Base url of the real data location
/// * `is_reading_segments` - Boolean representing the current state of reading, `true` if segments are currently being read
/// * `req` - The HTTP Request
async fn index(
    base_url: web::Data<Url>,
    is_reading_segments: web::Data<AtomicBool>,
    req: HttpRequest,
) -> HttpResponse {
    let target_url = match build_target_url(base_url.as_ref(), req.path()) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("URL building error: {}", err);
            return HttpResponse::BadRequest().finish();
        }
    };

    let guessed_file_type = type_guesser::guess_file_type(req.path());
    match (
        guessed_file_type,
        is_reading_segments.load(Ordering::SeqCst),
    ) {
        (Some(FileType::Manifest), true) => {
            println!("{}", "[TRACK SWITCH]".red());
            is_reading_segments.store(false, Ordering::SeqCst);
        }
        (Some(FileType::Segment), _) => {
            is_reading_segments.store(true, Ordering::SeqCst);
        }
        _ => {}
    }

    let guessed_info_str = guessed_file_type
        .as_ref()
        .map(FileType::uppercase_string)
        .map(|ft| format!("[{}]", ft))
        .unwrap_or_default();

    println!(
        "{}{} {}",
        "[IN]".red(),
        guessed_info_str.red(),
        target_url.to_string().purple()
    );

    let now = std::time::Instant::now();
    let body = fetch_url(target_url.clone()).await;
    let elapsed_ms = now.elapsed().as_millis();

    match body {
        Ok(body) => {
            let body = match (guessed_file_type, String::from_utf8(body.clone())) {
                (Some(FileType::Manifest), Ok(body_str)) => {
                    url_rewriting::rewrite_manifest(&body_str)
                        .as_bytes()
                        .to_vec()
                }
                _ => body,
            };

            let elapsed_str = format!("({}ms)", elapsed_ms);
            println!(
                "{}{} {} {}",
                "[OUT]".red(),
                guessed_info_str.red(),
                target_url.to_string().purple(),
                elapsed_str.purple()
            );
            HttpResponse::Ok().body(body)
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            HttpResponse::BadRequest().finish()
        }
    }
}

/// Returns the `base_url` from the command line arguments
///
/// # Errors
/// If no URL was provided in the command line or if the URL cannot be parsed this function will return an error
fn parse_url_from_args() -> Result<Url, &'static str> {
    let url_str = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        return Err("Please provide an URL for the base of the playlist");
    };

    let url = if let Ok(url) = reqwest::Url::parse(&url_str) {
        url
    } else {
        return Err("Cannot parse the provided URL");
    };

    Ok(url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = match parse_url_from_args() {
        Ok(url) => url,
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    };
    println!("Loading base URL: {}", url);

    let base_url = web::Data::new(url);
    let is_reading_segments = web::Data::new(AtomicBool::new(false));

    HttpServer::new(move || {
        App::new()
            .app_data(base_url.clone())
            .app_data(is_reading_segments.clone())
            .default_service(web::route().to(index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
