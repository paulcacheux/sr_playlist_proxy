use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use colored::Colorize;
use reqwest::Url;
use std::sync::atomic::{AtomicBool, Ordering};

mod type_guesser;
use type_guesser::FileType;

async fn fetch_url(url: Url) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;
    Ok(body.into_iter().collect())
}

async fn index(
    base_url: web::Data<Url>,
    is_reading_segments: web::Data<AtomicBool>,
    req: HttpRequest,
) -> HttpResponse {
    let target_url = match base_url.join(req.path()) {
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
