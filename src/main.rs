use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use colored::Colorize;
use reqwest::Url;

async fn fetch_url(url: Url) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;
    Ok(body.into_iter().collect())
}

async fn index(base_url: web::Data<Url>, req: HttpRequest) -> HttpResponse {
    let target_url = match base_url.join(req.path()) {
        Ok(url) => url,
        Err(err) => {
            eprintln!("URL building error: {}", err);
            return HttpResponse::BadRequest().finish();
        }
    };

    println!("{} {}", "[IN]".red(), target_url.to_string().purple());

    let now = std::time::Instant::now();
    let body = fetch_url(target_url.clone()).await;
    let elapsed_ms = now.elapsed().as_millis();

    match body {
        Ok(body) => {
            let elapsed_str = format!("({}ms)", elapsed_ms);
            println!(
                "{} {} {}",
                "[OUT]".red(),
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

    HttpServer::new(move || {
        App::new()
            .app_data(base_url.clone())
            .default_service(web::route().to(index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
