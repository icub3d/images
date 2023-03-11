use std::path::Path;

use images::{Images, Media};

use clap::Parser;
use tokio;

/// Manage an instance of Images.
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

    /// The database name to us for the images database.
    #[clap(short, long, env = "IMAGES_DB", default_value = "images")]
    db: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Put(Put),
    List(List),
    Get(Get),
    Remove(Remove),
}

/// Add new media to an instance of Images.
#[derive(Parser, Debug)]
struct Put {
    filename: String,
    typ: String,
    tags: String,
}

/// List media from an instance of Images.
#[derive(Parser, Debug)]
struct List {
    search: String,
    #[clap(short, long, default_value = "0")]
    skip: u64,
    #[clap(short, long, default_value = "100")]
    limit: i64,
}

/// Get metadata about media from an instance of Images.
#[derive(Parser, Debug)]
struct Get {
    id: String,
}

/// Remove media from an instance of Images.
#[derive(Parser, Debug)]
struct Remove {
    id: String,
}

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let mut images = Images::open(&options.uri, &options.db).await.unwrap();

    match options.subcmd {
        SubCommand::Put(p) => {
            let tags = p
                .tags
                .split(',')
                .map(|t| t.trim().to_string())
                .collect::<Vec<String>>();
            images
                .put(&mut Media {
                    typ: p.typ,
                    filename: Path::new(&p.filename.clone())
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    tags,
                })
                .await
                .unwrap();
        }
        SubCommand::Get(g) => match images.get(&g.id).await.unwrap() {
            Some(m) => println!("{:?}", m),
            None => println!("not found"),
        },
        SubCommand::Remove(r) => images.remove(&r.id).await.unwrap(),
        SubCommand::List(l) => {
            for m in images.list(l.skip, l.limit, &l.search).await.unwrap() {
                println!("{} {:?}", m.filename, m.tags);
            }
        }
    }
}
