use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn index(_req: HttpRequest) -> HttpResponse {
    let now = std::time::Instant::now();

    let body =
        fetch("https://bitdash-a.akamaihd.net/content/MI201109210084_1/m3u8s/f08e80da-bf1d-4e3d-8899-f0f6155f6efa.m3u8")
            .await;

    match body {
        Ok(body) => {
            println!("request time: {} ms", now.elapsed().as_millis());
            HttpResponse::Ok().content_type("image/jpeg").body(body)
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        eprintln!("Please provide an URL for the base of the playlist");
        std::process::exit(1);
    };

    println!("URL: {}", url);
    let port = 3000;

    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
