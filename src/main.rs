mod table;
mod config;
mod latex;

use actix_multipart::Multipart;
use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error};
use futures_util::StreamExt as _;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;

use table::parser::load_csv;
use config::load_config;
use latex::LaTeXTable;

async fn render_latex(mut payload: Multipart) -> Result<impl Responder, Error> {
    let mut files = vec![];

    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap().to_string();

        let filepath = PathBuf::from(format!("/tmp/{}", filename));
        let mut f = File::create(&filepath)?;

        while let Some(chunk_result) = field.next().await {
            let chunk = chunk_result?;
            f.write_all(&chunk)?;
        }

        files.push(filepath);
    }

    if files.len() != 2 {
        return Ok(HttpResponse::BadRequest().body("Please upload exactly two files."));
    }

    let (csv_path, config_path) = if files[0].extension().unwrap() == "csv" {
        (files[0].clone(), files[1].clone())
    } else {
        (files[1].clone(), files[0].clone())
    };

    let table = load_csv(csv_path.to_str().unwrap()).map_err(actix_web::error::ErrorInternalServerError)?;
    let config = load_config(config_path.to_str().unwrap()).map_err(actix_web::error::ErrorInternalServerError)?;
    let latex_table = LaTeXTable::from_table(&table, &config);

    Ok(HttpResponse::Ok().content_type("text/plain").body(latex_table.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/render", web::post().to(render_latex))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}