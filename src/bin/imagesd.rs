use futures::lock::Mutex;

use images::{Images, Media};

#[macro_use]
extern crate rocket;
use clap::Parser;
use rocket::{fs::FileServer, http::Status, response::status, serde::json::Json, State};

/// Serve Images over an HTTP server.
#[derive(Parser, Debug)]
#[clap(name = "Images")]
struct Options {
    /// The MongoDB URI to use for the images database.
    #[clap(
        short,
        long,
        env = "IMAGES_URI",
        default_value = "mongodb://localhost:27017"
    )]
    uri: String,

    /// The path to the static files for the UI.
    #[clap(long, env = "IMAGES_UI", default_value = "./ui")]
    ui: String,

    /// The database name to us for the images database.
    #[clap(short, long, env = "IMAGES_DB", default_value = "images")]
    db: String,
}

#[get("/media?<skip>&<limit>&<search>")]
async fn get(
    images: &State<Mutex<Images>>,
    skip: Option<u64>,
    limit: Option<i64>,
    search: Option<&str>,
) -> Result<Json<Vec<Media>>, status::Custom<String>> {
    let skip = skip.unwrap_or(0);
    let limit = limit.unwrap_or(0);
    let search = search.unwrap_or("");
    let mut images = images.lock().await;
    match images.list(skip, limit, search).await {
        Ok(mm) => Ok(Json(mm)),
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}

#[launch]
async fn rocket() -> _ {
    let options = Options::parse();

    let images = Images::open(&options.uri, &options.db).await.unwrap();

    rocket::build()
        .mount("/", FileServer::from(options.ui))
        .mount("/api", routes![get])
        .manage(Mutex::new(images))
}
