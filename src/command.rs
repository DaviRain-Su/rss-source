use crate::constant::DEFAULT_CONFIG_FILE;
use git2::{Cred, PushOptions, RemoteCallbacks};
use git2::{Oid, Repository, RepositoryInitOptions, Signature};
use std::fs;
use std::path::Path;
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
        html_link: String,
        #[structopt(short, long)]
        xml_link: String,
    },
    /// Remove an item from the RSS feed by xml link
    Remove {
        #[structopt(short, long)]
        xml_link: String,
    },
    /// auto generate default.xml
    Auto(Auto),

    /// copy default.xml file to target dir
    CopyToFile { target_path: PathBuf },

    /// uploda to github
    Upload {
        #[structopt(short, long)]
        message: Option<String>,
    },
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

pub fn upload_to_github(message: Option<&str>) -> anyhow::Result<()> {
    let home_path = dirs::home_dir().ok_or(anyhow::anyhow!("can't open home dir"))?;
    let repo_path = home_path.join(".config").join("rssss");
    ensure_git_initialized(&repo_path)?;
    let repo = Repository::open(&repo_path)?;

    // Check if the 'rssss' remote exists
    let mut remote = match repo.find_remote("rssss") {
        Ok(remote) => remote,
        Err(_) => {
            // If 'rssss' remote doesn't exist, add it
            repo.remote("rssss", "https://github.com/DaviRain-Su/rssss.git")?;
            // Fetch the new remote to get the updated reference
            repo.find_remote("rssss")?
        }
    };

    let mut index = repo.index()?;
    index.add_path(Path::new("default.xml"))?; // replace with the relative path to default.xml in your repo
    let oid = index.write_tree()?;

    let tree = repo.find_tree(oid)?;
    let head = repo.head()?.peel_to_commit()?;
    let signature = Signature::now("DaviRain-Su", "davirian.yin@gmail.com")?;

    let message = message.unwrap_or("Update default.xml");
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&head],
    )?;

    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        // Replace with your SSH key path and passphrase (if any)
        let home_path = dirs::home_dir().unwrap();
        let ssh_key_path = home_path.join(".ssh/id_rsa");
        let username = username_from_url.unwrap_or("DaviRain-Su");
        Cred::ssh_key(username, None, ssh_key_path.as_path(), None)
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.push(&["refs/heads/main"], Some(&mut push_options))?;

    Ok(())
}

pub fn ensure_git_initialized(repo_path: &Path) -> anyhow::Result<()> {
    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        // Initialize a new git repository if it doesn't already exist
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main"); // Set the default branch to 'main'
        opts.mkpath(true); // Make parent directories as necessary
        Repository::init_opts(repo_path, &opts)?;
    }
    Ok(())
}

pub fn create_initial_commit(repo: &Repository) -> anyhow::Result<Oid> {
    let mut index = repo.index()?;

    // Add the default.xml file to the index
    // Assumes default.xml is in the root of the repository
    index.add_path(Path::new("default.xml"))?;

    // Write the index to create a tree object
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    let signature = Signature::now("DaviRain-Su", "davirian.yin@gmail.com")?;

    let message = "Initial commit";

    let oid = repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;

    // Create a new branch named 'main' pointing to the initial commit
    repo.branch("main", &repo.find_commit(oid)?, false)?;

    // Update HEAD to point to the 'main' branch
    repo.reference_symbolic("HEAD", "refs/heads/main", true, "Initial HEAD")?;

    Ok(oid)
}
