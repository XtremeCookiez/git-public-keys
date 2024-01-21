use config::Config;
use std::{collections::HashSet, env::temp_dir, hash::Hash};
use git2::Repository;
use tempdir::TempDir;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher};
use std::path::Path;

fn debug_print(debug: bool, msg: String) {
    if debug {
        println!("{}", msg);
    }
}

fn main() {
    let debug: bool = true;

    let config: Config = Config::builder()
        .add_source(config::File::with_name("/etc/git-public-keys/config").required(false))
        .add_source(config::File::with_name("config").required(false))
        .build()
        .unwrap();

    let urls = config.get::<Vec<String>>("repos.urls").unwrap();
    debug_print(debug, format!("{:?}", urls));

    for url in urls {
        let mut s = DefaultHasher::new();
        url.hash(&mut s);
        let hash = format!("{:x}", s.finish());
        let tmp_dir = TempDir::new(&hash).unwrap();
        debug_print(debug, format!("{:?}", tmp_dir.path()));

        let repo = git2::Repository::clone(&url, Path::new("tmp/pubkeys")).unwrap();
        let tmp_dir_path = tmp_dir.path();

        let auth_keys = tmp_dir_path.join("authorized_keys");
        for entry in tmp_dir_path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                println!("LS: {:?}", entry.path());
            }
        }
        println!("{:?}", auth_keys);
        if auth_keys.exists() {
            println!("neat");
        }
        // if 
    }
}
