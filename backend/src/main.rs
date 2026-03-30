mod db;
mod models;
mod audio;
mod services;
mod handlers;

use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::Arc;
use crate::db::Database;
use crate::handlers::*;

const DEFAULT_DB_PATH: &str = "data/playlists.db";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("data").ok();

    let db = match Database::new(DEFAULT_DB_PATH) {
        Ok(db) => Arc::new(db),
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };

    println!("Starting Dynamic Playlist Generator server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/health", web::get().to(health_check))
            .route("/api/tracks", web::post().to(upload_track))
            .route("/api/tracks", web::get().to(list_tracks))
            .route("/api/tracks/{id}", web::get().to(get_track))
            .route("/api/playlists", web::post().to(create_playlist))
            .route("/api/playlists", web::get().to(list_playlists))
            .route("/api/playlists/{id}", web::get().to(get_playlist))
            .route("/api/similarities/recompute", web::post().to(recompute_similarities))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
