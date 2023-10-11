use opml::OPML;

pub const DEFAULT_CONFIG_FILE: &str = r#"
<opml version="2.0">
    <head>
        <title>Your Subscription List</title>
    </head>
    <body>
        <outline text="左耳朵耗子blog" type="rss" xmlUrl="https://coolshell.cn/feed" htmlUrl="https://coolshell.cn/"/>
    </body>
</opml>
"#;

fn main() {
    let document = OPML::from_str(DEFAULT_CONFIG_FILE).unwrap();

    assert_eq!(document.version, "2.0");

    println!("{document:#?}");
}
