use actix_web::{web, App, HttpServer, HttpResponse, Result, Error, http::header};
use tera::{Tera, Context};
use serde::Deserialize;
use std::io;
use std::fs::File;
use std::io::Read;
use urlencoding::encode;

#[derive(Deserialize)]
struct YoutubeLinkFormData {
    youtube_link: String,
}

#[derive(Deserialize)]
struct DownloadQuery {
    file_path: String,
}

async fn index() -> HttpResponse {
    println!("Index");
    let tera: Tera = Tera::new("templates/**/**").unwrap();
    let rendered: String = tera.render("main.html", &Context::new()).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn download_video(req: web::Form<YoutubeLinkFormData>) -> HttpResponse {
    println!("download_video");
    println!("Youtube link is: {}", req.youtube_link);
    let file_path = rustube::download_best_quality(&*req.youtube_link).await.unwrap();

    let download_link = format!("/downloaded_file?file_path={}", encode(file_path.to_str().unwrap()));
    let mut context = Context::new();
    context.insert("download_link", &download_link);

    let tera: Tera = Tera::new("templates/**/**").unwrap();
    let rendered = tera.render("download_link.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
    // HttpResponse::Ok().body(format!("File downloaded successfully! <a href='{}'>Click here to download</a>", download_link))
}

async fn download_file(query: web::Query<DownloadQuery>) -> Result<HttpResponse, Error> {
    let file_path = query.file_path.clone(); // Assuming DownloadQuery has a field file_path
    match File::open(file_path) {
        Ok(mut file) => {
            let mut bytes = vec![];
            file.read_to_end(&mut bytes).expect("Cannot read file.");
            Ok(HttpResponse::Ok()
                .content_type("application/octet-stream")
                .append_header((header::CONTENT_DISPOSITION, format!("attachmend; filename=\"{}\"", query.file_path)))
                .body(web::Bytes::from(bytes)))
        },
        Err(_) => Err(Error::from(io::Error::new(io::ErrorKind::NotFound, "File not found")))
    }
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/",  web::get().to(index))
            .route("/download", web::post().to(download_video))
            .route("/downloaded_file", web::get().to(download_file))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
