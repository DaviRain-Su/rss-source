use opml::{Outline, OPML};
use std::fs::File;
use std::io::{self, Write};

pub const DEFAULT_CONFIG_FILE: &str = r#"
<opml version="2.0">
    <head>
        <title>Your Subscription List</title>
    </head>
    <body>
        <outline text="24 ways" htmlUrl="http://24ways.org/" type="rss" xmlUrl="http://feeds.feedburner.com/24ways"/>
    </body>
</opml>
"#;

#[derive(Debug)]
pub struct RssClient {
    document: OPML,
}

impl RssClient {
    pub fn new(opml_str: &str) -> Self {
        let document = OPML::from_str(opml_str).expect("Failed to parse OPML data");
        RssClient { document }
    }

    pub fn add_subscription(&mut self, text: &str, html_url: &str, xml_url: &str) {
        let subscription_outline = Outline {
            text: text.to_string(),
            r#type: Some("rss".to_string()),
            html_url: Some(html_url.to_string()),
            xml_url: Some(xml_url.to_string()),
            ..Default::default() // Fill other fields with default values
        };
        self.document.body.outlines.push(subscription_outline);
    }

    pub fn remove_subscription_by_xml_url(&mut self, xml_url: &str) {
        self.document
            .body
            .outlines
            .retain(|subscription| subscription.xml_url.as_deref() != Some(xml_url));
    }

    pub fn write_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        write!(file, "{}", self.document.to_string().unwrap())
    }
}
