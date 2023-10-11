#![allow(unused_variables)]
use rss::validation::Validate;
use rss::Channel;
use rss::ItemBuilder;
use std::fs::File;
use std::io::prelude::*;

use std::str::FromStr;

#[derive(Default, Debug)]
pub struct RssSourceClient {
    channel: Channel,
}

impl RssSourceClient {
    pub fn validate(&self) -> bool {
        self.channel.validate().is_ok()
    }

    pub fn add_item(&mut self, title: &str, link: &str, description: &str) {
        let item = ItemBuilder::default()
            .title(Some(title.into()))
            .link(Some(link.into()))
            .description(Some(description.into()))
            .build();

        // Get the existing items, add the new item, and set the updated items list on the channel
        let mut items_vec: Vec<rss::Item> = self.channel.items().to_vec();
        items_vec.push(item);
        self.channel.set_items(items_vec);
    }

    pub fn remove_item(&mut self, title: &str) {
        let items_vec: Vec<rss::Item> = self.channel.items().to_vec();
        let items_vec = items_vec
            .into_iter()
            .filter(|item| item.title() != Some(title))
            .collect::<Vec<_>>();
        // Assuming there's a method to set the items Vec on the Channel object
        self.channel.set_items(items_vec);
    }

    pub fn write_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(self.channel.to_string().as_bytes())?;
        Ok(())
    }
}

impl TryFrom<&str> for RssSourceClient {
    type Error = anyhow::Error;

    fn try_from(contents: &str) -> Result<Self, Self::Error> {
        let channel = Channel::from_str(contents)?;
        Ok(Self { channel })
    }
}

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[test]
fn test_read_channel() {
    let channel = read_file("./data/channel.xml").unwrap();
    let rss_source = RssSourceClient::try_from(channel.as_str()).unwrap();

    println!("rss_source: {:#?}", rss_source);
}

#[test]
fn test_constant_rss_file() {
    let static_file = crate::constant::DEFAULT_CONFIG_FILE;
    let rss_source = RssSourceClient::try_from(static_file).unwrap();

    println!("rss_source valid: {:#?}", rss_source.validate());
}
