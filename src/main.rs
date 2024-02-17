use config::Config;
use std::collections;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

use std::process::Command;
use std::{collections::HashSet, env::temp_dir, hash::Hash};
use tempdir::TempDir;

fn debug_print(debug: bool, msg: String) {
    if debug {
        println!("{}", msg);
    }
}

fn read_key_file(filename: PathBuf) -> io::Result<collections::HashSet<String>> {
    let mut keys = HashSet::new();
    if filename.exists() {
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();
        for key in lines {
            keys.insert(key.unwrap());
        }
    }

    return Ok(keys);
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

    let auth_keys_path = config
        .get::<String>("authorized_keys.path")
        .map(|s| PathBuf::from(s))
        .unwrap();

    let system_keys = read_key_file(auth_keys_path).unwrap();
    let mut merged_keys: HashSet<String> = HashSet::from(system_keys);
    for url in urls {
        let mut s = DefaultHasher::new();
        url.hash(&mut s);
        let hash = format!("{:x}", s.finish());
        let tmp_dir = TempDir::new(&hash).unwrap();
        debug_print(debug, format!("{:?}", tmp_dir.path()));

        let mut git_clone = Command::new("git");
        git_clone.args(["clone", &url, tmp_dir.path().to_str().unwrap()]);
        git_clone.output().unwrap();

        let tmp_dir_path = tmp_dir.path();

        let auth_keys = tmp_dir_path.join("authorized_keys");

        if auth_keys.exists() {
            let new_keys = read_key_file(auth_keys).unwrap();
            merged_keys.extend(new_keys);
        }
    }

    for k in merged_keys {
        debug_print(debug, k);
    }
}
