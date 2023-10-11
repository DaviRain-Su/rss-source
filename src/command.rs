use crate::constant::DEFAULT_CONFIG_FILE;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rssss", about = "A tool for managing RSS feeds.")]
pub enum Opt {
    /// Add a new item to the RSS feed
    Add {
        #[structopt(short, long)]
        title: String,
        #[structopt(short, long)]
        link: String,
        #[structopt(short, long)]
        description: String,
    },
    /// Remove an item from the RSS feed by title
    Remove {
        #[structopt(short, long)]
        title: String,
    },
    /// auto generate default.xml
    Auto(Auto),

    /// copy default.xml file to target dir
    CopyToFile { target_path: PathBuf },
}

#[derive(Debug, StructOpt)]
pub struct Auto {
    /// config path for Phoenix onchain Maket Maker
    #[structopt(short, long)]
    config_path: Option<PathBuf>,
}

impl Auto {
    pub fn run(&self) -> anyhow::Result<PathBuf> {
        if let Some(config_path) = self.config_path.clone() {
            println!("enpter input config file");
            Ok(config_path)
        } else {
            // open  config file path is  ~/.config/rss-/config.toml
            let home_path = dirs::home_dir().ok_or(anyhow::anyhow!("can't open home dir"))?;
            let pomm_config_path = home_path.join(".config").join("rssss");
            let config_path = pomm_config_path.join("default.xml");
            if std::fs::read_to_string(config_path.clone()).is_ok() {
                Ok(config_path)
            } else {
                std::fs::create_dir_all(pomm_config_path.clone())?;
                let config_path = pomm_config_path.join("default.xml");
                std::fs::write(config_path.clone(), DEFAULT_CONFIG_FILE)?;
                Ok(config_path)
            }
        }
    }
}

pub fn copy_default_toml_to_target(target_path: PathBuf) -> anyhow::Result<()> {
    let home_path = dirs::home_dir().ok_or(anyhow::anyhow!("can't open home dir"))?;
    let config_path = home_path.join(".config").join("rssss");
    let source_path = config_path.join("default.xml");

    let final_target_path = if target_path.is_dir() {
        // 如果 target_path 是一个目录，则在该目录下创建一个与源文件名相同的新文件
        target_path.join(source_path.file_name().unwrap())
    } else {
        // 否则，使用 target_path 作为文件路径
        target_path
    };

    // 如果目标文件不存在，则确保目标目录存在
    if !final_target_path.exists() {
        fs::create_dir_all(final_target_path.parent().unwrap())?;
    }

    // 复制文件
    fs::copy(&source_path, &final_target_path)?;

    Ok(())
}
