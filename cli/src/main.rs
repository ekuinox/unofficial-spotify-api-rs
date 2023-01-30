use anyhow::{bail, ensure, Result};
use clap::Parser;
use itertools::Itertools;
use reqwest::Url;
use unofficial_spotify_api::Client;

#[derive(Parser, Debug)]
pub enum App {
    Credits { url: Url },
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::parse();

    match app {
        App::Credits { url } => {
            let Some(segments) = url.path_segments() else {
                bail!("Invalid url");
            };
            let Some((kind, id)) = segments.collect_tuple::<(_, _)>() else {
                bail!("Invalid url. segments length not matched");
            };
            ensure!(kind == "track", "Not track url");

            let client = Client::init().await?;
            let view = client.track_credits_view(id).await?;

            println!("Track: {}", view.track_title);
            for credit in &view.role_credits {
                if credit.artists.is_empty() {
                    continue;
                }
                println!("- Credit Subrole: {}", credit.role_title);
                for artist in &credit.artists {
                    let subroles = artist.subroles.join(", ");
                    println!("  + {} ... {}", artist.name, subroles);
                }
            }

            Ok(())
        }
    }
}
