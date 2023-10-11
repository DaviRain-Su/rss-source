pub mod client;
pub mod command;
pub mod constant;
pub mod opml_client;

use command::Opt;
use opml_client::RssClient;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Add {
            title,
            html_link,
            xml_link,
        } => {
            let home_path = dirs::home_dir().ok_or(anyhow::anyhow!("can't open home dir"))?;
            let pomm_config_path = home_path.join(".config").join("rssss");
            let config_path = pomm_config_path.join("default.xml");
            let channel = client::read_file(config_path.to_str().unwrap())?;
            let mut client = RssClient::new(channel.as_str())?;
            // Assume you have a new method or use an existing method to create a client
            client.add_subscription(&title, &html_link, &xml_link);
            // Optionally, write back to file immediately
            client.write_to_file(config_path.to_str().unwrap())?;
        }
        Opt::Remove { xml_link } => {
            let home_path = dirs::home_dir().ok_or(anyhow::anyhow!("can't open home dir"))?;
            let pomm_config_path = home_path.join(".config").join("rssss");
            let config_path = pomm_config_path.join("default.xml");
            let channel = client::read_file(config_path.to_str().unwrap())?;
            let mut client = RssClient::new(channel.as_str())?;
            client.remove_subscription_by_xml_url(&xml_link);
            // Optionally, write back to file immediately
            client.write_to_file(config_path.to_str().unwrap())?;
        }
        Opt::Auto(cmd) => match cmd.run() {
            Ok(path) => {
                println!("generate path is {}", path.display());
            }
            Err(e) => {
                eprintln!("auto generate default.xml file failed({})", e);
            }
        },

        Opt::CopyToFile { target_path } => command::copy_default_toml_to_target(target_path)?,
    }

    Ok(())
}
