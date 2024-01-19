use std::error::Error;

use chrono::{DateTime, Duration, Utc};
use dotenv;
use rss::Channel;
use serde::{Deserialize, Serialize};

async fn fetch_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://github.blog/changelog/feed/")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

struct Content {
    title: String,
    link: String,
    pub_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DiscordPost {
    // https://discord.com/developers/docs/resources/webhook#execute-webhook
    content: String,
    embeds: [Embed; 1],
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Embed {
    // https://discord.com/developers/docs/resources/channel#embed-object
    #[serde(rename = "type")]
    embed_type: String,
    title: String,
    url: String,
}

async fn post_to_discord(contents: &Vec<Content>) {
    let webhook_url: &str = &dotenv::var("WEBHOOK_URL").unwrap();

    let client = reqwest::Client::new();

    for content in contents {
        let post = DiscordPost {
            // Username: "GitHub Changelog".to_string(),
            content: content.title.clone(),
            embeds: [Embed {
                embed_type: "rich".to_string(),
                title: content.title.clone(),
                url: content.link.clone(),
            }],
        };
        let json = serde_json::to_string(&post);
        println!("json: {}", json.unwrap());
        let res = client.post(webhook_url).json(&post).send().await;
        match res {
            Ok(_) => println!("Sent a message"),
            Err(e) => println!("Got an error: {}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let channel = fetch_feed().await.unwrap();
    channel.categories.iter().for_each(|category| {
        println!("Category: {}", category.name);
    });

    let one_day_ago = Utc::now() - Duration::days(1);
    let contents = channel
        .items
        .iter()
        .map(|item| Content {
            title: item.title.clone().unwrap(),
            link: item.link.clone().unwrap(),
            pub_date: item.pub_date.clone().unwrap(),
        })
        .filter(|item| {
            let given_date = DateTime::parse_from_str(item.pub_date.as_str(), "%a, %d %b %Y %T %z")
                .expect("Failed to parse given date");

            given_date > one_day_ago
        })
        .collect::<Vec<Content>>();

    post_to_discord(&contents).await;

    for item in channel.items {
        println!("================= ================");
        println!("Title: {}", item.title.unwrap());
        println!("Link: {}", item.link.unwrap());
        println!("Date: {}", item.pub_date.unwrap());
        println!("Author: {:?}", item.author);
        println!("Enclosure: {:?}", item.enclosure);
    }
}
