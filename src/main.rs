use std::{collections::HashMap, io::BufReader};

use atom_syndication::Feed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .unwrap();
    let settings = settings.try_into::<HashMap<String, String>>().unwrap();
    let content = reqwest::get(settings.get("url").unwrap())
        .await?
        .bytes()
        .await?;
    let feed = Feed::read_from(BufReader::new(&content[..])).unwrap();

    // let string = "<feed></feed>";
    // let feed = string.parse::<Feed>().unwrap();
    println!("{:#?}", feed.entries[0]);
    
    Ok(())
}